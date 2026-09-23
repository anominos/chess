use backend::consts::{BISHOP_RELEVANCY, ROOK_RELEVANCY};
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    error::Error,
    fs,
    path::PathBuf,
    str::FromStr,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time,
};
use syn;

const ROOK_DIRS: [(i32, i32); 4] = [
    (1, 0),  // north
    (-1, 0), // south
    (0, 1),  // east
    (0, -1), // west
];

const BISHOP_DIRS: [(i32, i32); 4] = [
    (1, 1),   // NE
    (1, -1),  // NW
    (-1, 1),  // SE
    (-1, -1), // SW
];

const SHIFT_START: usize = 13;
const MAX_ARR_SIZE: usize = 8192;

// Tuple of (blocker, attacks) for each square
static BISHOP_BLOCKER_ATTACKS: LazyLock<[Vec<(u64, u64)>; 64]> = LazyLock::new(|| {
    std::array::from_fn(|square| {
        let relevancy = BISHOP_RELEVANCY[square];
        let mut blockers = relevancy;
        (0..1usize << relevancy.count_ones())
            .map(|_| {
                blockers = blockers.wrapping_sub(1) & relevancy;
                let attacks = calculate_attacks(square, blockers, &BISHOP_DIRS);
                (blockers, attacks)
            })
            .collect()
    })
});

static ROOK_BLOCKER_ATTACKS: LazyLock<[Vec<(u64, u64)>; 64]> = LazyLock::new(|| {
    std::array::from_fn(|square| {
        let relevancy = ROOK_RELEVANCY[square];
        let mut blockers = relevancy;
        (0..1usize << relevancy.count_ones())
            .map(|_| {
                blockers = blockers.wrapping_sub(1) & relevancy;
                let attacks = calculate_attacks(square, blockers, &ROOK_DIRS);
                (blockers, attacks)
            })
            .collect()
    })
});

struct Magic {
    magic: u64,
    mask: u64,
    shift: usize,
    array: Vec<Option<u64>>, // array[blockers * magic >> shift] = attackers
}

fn main() {
    let (b_magic, r_magic) = load_magics()
        .unwrap_or_else(|_| (std::array::from_fn(|_| None), std::array::from_fn(|_| None)));

    let b_magic = Arc::new(b_magic.map(|x| Mutex::new(x)));
    let r_magic = Arc::new(r_magic.map(|x| Mutex::new(x)));

    let running = Arc::new(AtomicBool::new(true));
    ctrlc::set_handler({
        let running = running.clone();
        move || {
            running.store(false, Ordering::Relaxed);
        }
    })
    .unwrap();

    let total_tries = Arc::new(AtomicU64::new(0));

    let num_threads = std::thread::available_parallelism().unwrap().get();

    let threads: Vec<_> = (0..num_threads)
        .map(|thread_num| {
            let running = Arc::clone(&running);
            let r_magic = Arc::clone(&r_magic);
            let b_magic = Arc::clone(&b_magic);
            let total_tries = Arc::clone(&total_tries);
            let now = time::Instant::now();
            std::thread::spawn(move || {
                let mut rng = rand::rng();
                let mut c = 0;
                while running.load(Ordering::Relaxed) {
                    let magic = gen_rng(&mut rng);
                    for sq in 0..64 {
                        for is_rook in [true, false] {
                            // acquire lock to read cur len and shift, and immediately drop lock
                            // use these values as estimates, a proper compare is done when a better value is found
                            let magic_array = if is_rook { &r_magic } else { &b_magic };
                            let (cur_len, cur_shift) =
                                if let Some(magic) = &*magic_array[sq].lock().unwrap() {
                                    (magic.array.len(), magic.shift)
                                } else {
                                    (1 << SHIFT_START, 64 - SHIFT_START)
                                };
                            for shift in cur_shift..=61 {
                                if let Some(arr) = check_magic(magic, sq, shift, is_rook) {
                                    let trimmed_arr = trim_trailing_nones(arr);
                                    if trimmed_arr.len() < cur_len {
                                        // Array is probably better, acquire lock to check and write to arr
                                        // also update cur_len,  but cur_shift can be left unchanged because its only used in loop
                                        let mut entry = magic_array[sq].lock().unwrap();
                                        if entry
                                            .as_ref()
                                            .is_none_or(|m| trimmed_arr.len() < m.array.len())
                                        {
                                            *entry = Some(Magic {
                                                magic: magic,
                                                mask: if is_rook {
                                                    ROOK_RELEVANCY
                                                } else {
                                                    BISHOP_RELEVANCY
                                                }[sq],
                                                shift: shift,
                                                array: trimmed_arr,
                                            });
                                            println!(
                                                "Better magic found: popcount {} sq {}",
                                                magic.count_ones(),
                                                sq
                                            );
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    c += 1;
                    if c % 2000 == 0 {
                        total_tries.fetch_add(2000, Ordering::Relaxed);
                        if thread_num == 1 {
                            let r_arr_len: usize = r_magic
                                .iter()
                                .map(|x| x.lock().unwrap().as_ref().map_or(0, |m| m.array.len()))
                                .sum();
                            let b_arr_len: usize = b_magic
                                .iter()
                                .map(|x| x.lock().unwrap().as_ref().map_or(0, |m| m.array.len()))
                                .sum();
                            let elapsed = now.elapsed().as_millis() as f64 / 1000.0;

                            println!(
                                "Try {}, r={r_arr_len}, b={b_arr_len}, time={elapsed:.2}s",
                                total_tries.load(Ordering::Relaxed)
                            )
                        }
                    }
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    let b_magic = Arc::into_inner(b_magic)
        .unwrap()
        .map(|m| m.into_inner().unwrap());
    let r_magic = Arc::into_inner(r_magic)
        .unwrap()
        .map(|m| m.into_inner().unwrap());
    write_magics(b_magic, r_magic).expect("Write failed, missing values in magic");
}

fn gen_rng(rng: &mut impl rand::Rng) -> u64 {
    let bits =
        (rand::random_range(0..64) + rand::random_range(0..64) + rand::random_range(0..64)) / 3;
    rand::seq::index::sample(rng, 64, bits)
        .into_iter()
        .fold(0u64, |acc, bit| acc | (1u64 << bit))
}

fn check_magic(
    magic: u64,
    square: usize,
    shift: usize,
    is_rook: bool,
) -> Option<[Option<u64>; MAX_ARR_SIZE]> {
    let blocker_attacks = if is_rook {
        &*ROOK_BLOCKER_ATTACKS
    } else {
        &*BISHOP_BLOCKER_ATTACKS
    };
    let mut array: [Option<u64>; MAX_ARR_SIZE] = [None; MAX_ARR_SIZE];
    for (blocker, attack) in &blocker_attacks[square] {
        let hash = (blocker.wrapping_mul(magic) >> shift) as usize;
        if let Some(other_attack) = array[hash] {
            if other_attack != *attack {
                return None;
            }
        } else {
            array[hash] = Some(*attack);
        }
    }
    Some(array)
}

fn calculate_attacks(square: usize, blockers: u64, directions: &[(i32, i32)]) -> u64 {
    let mut attacks = 0;

    let rank = (square / 8) as i32;
    let file = (square % 8) as i32;

    for &(dr, df) in directions {
        let mut r = rank + dr;
        let mut f = file + df;

        while (0..8).contains(&r) && (0..8).contains(&f) {
            let target = (r * 8 + f) as usize;

            attacks |= 1u64 << target;

            if blockers & (1u64 << target) != 0 {
                break;
            }

            r += dr;
            f += df;
        }
    }

    attacks
}

fn trim_trailing_nones(array: [Option<u64>; MAX_ARR_SIZE]) -> Vec<Option<u64>> {
    let end = array.iter().rposition(|x| x.is_some()).map_or(0, |i| i + 1);
    array[..end].to_vec()
}

fn magic_to_tokens(magic: [Option<Magic>; 64]) -> Result<Vec<TokenStream>, ()> {
    magic
        .iter()
        .map(|magic| {
            let magic = magic.as_ref().ok_or(())?;
            let magic_val = magic.magic;
            let magic_mask = magic.mask;
            let magic_shift = magic.shift;
            let magic_arr = magic.array.iter().map(|x| x.unwrap_or(0));
            Ok(quote! {
                Magic {
                    magic: #magic_val,
                    mask: #magic_mask,
                    shift: #magic_shift,
                    array: &[#(#magic_arr),*],
                }
            })
        })
        .collect::<Result<Vec<_>, ()>>()
}

fn write_magics(b_magic: [Option<Magic>; 64], r_magic: [Option<Magic>; 64]) -> Result<(), ()> {
    fs::create_dir_all(get_path().parent().unwrap()).unwrap();
    let b_magic = magic_to_tokens(b_magic)?;
    let r_magic = magic_to_tokens(r_magic)?;

    let tokens = quote! {
        // Generated file, see src/bin/gen_magic.rs
        pub struct Magic {
            pub magic: u64,
            pub mask: u64,
            pub shift: usize,
            pub array: &'static[u64]
        }
        pub const BISHOP_MAGICS: [Magic; 64] = [#(#b_magic),*];
        pub const ROOK_MAGICS: [Magic; 64] = [#(#r_magic),*];
    };
    let file = syn::parse2::<syn::File>(tokens).unwrap();
    let code = prettyplease::unparse(&file);
    fs::write(get_path(), code).unwrap();
    Ok(())
}

fn get_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/generated/magics.rs")
}

fn load_magics() -> Result<([Option<Magic>; 64], [Option<Magic>; 64]), Box<dyn Error>> {
    if !get_path().exists() {
        return Err("Path doesn't exist".into());
    }
    let code = std::fs::read_to_string(get_path())?;
    let file = syn::parse_file(&code)?;
    let mut b_magic: Option<[Option<Magic>; 64]> = None;
    let mut r_magic: Option<[Option<Magic>; 64]> = None;
    for item in &file.items {
        if let syn::Item::Const(c) = item {
            if c.ident == "BISHOP_MAGICS" {
                b_magic = Some(parse_value(c.expr.as_ref())?);
            }
            if c.ident == "ROOK_MAGICS" {
                r_magic = Some(parse_value(c.expr.as_ref())?);
            }
        }
    }
    Ok((
        b_magic.ok_or_else(|| "No bishop".to_string())?,
        r_magic.ok_or_else(|| "No rook".to_string())?,
    ))
}

fn parse_value(expr: &syn::Expr) -> Result<[Option<Magic>; 64], Box<dyn Error>> {
    let expr = match expr {
        syn::Expr::Array(s) => s,
        _ => return Err("Not an array".into()),
    };
    Ok(expr
        .elems
        .iter()
        .map(parse_magic_struct)
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| "len not 64")?)
}

fn parse_magic_struct(expr: &syn::Expr) -> Result<Option<Magic>, Box<dyn Error>> {
    let expr = match expr {
        syn::Expr::Struct(s) => s,
        _ => return Err("Not a struct".into()),
    };
    let mut magic_val = None;
    let mut magic_mask = None;
    let mut magic_shift = None;
    let mut magic_arr = None;
    for field in &expr.fields {
        let field_name = match &field.member {
            syn::Member::Named(name) => name.to_string(),
            _ => return Err("unnamed field".into()),
        };
        match field_name.as_str() {
            "magic" => magic_val = Some(parse_num::<u64>(&field.expr)?),
            "mask" => magic_mask = Some(parse_num::<u64>(&field.expr)?),
            "shift" => magic_shift = Some(parse_num::<usize>(&field.expr)?),
            "array" => magic_arr = Some(parse_u64_array(&field.expr)?),
            _ => return Err("Unknown field".into()),
        };
    }
    Ok(Some(Magic {
        magic: magic_val.ok_or("No magic")?,
        mask: magic_mask.ok_or("No mask")?,
        shift: magic_shift.ok_or("No shift")?,
        array: magic_arr.ok_or("No arr")?,
    }))
}

fn parse_num<T>(expr: &syn::Expr) -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(i),
            ..
        }) => Ok(i.base10_parse()?),
        _ => Err("Bad int lit".into()),
    }
}

fn parse_u64_array(expr: &syn::Expr) -> Result<Vec<Option<u64>>, Box<dyn Error>> {
    let arr = match expr {
        syn::Expr::Reference(r) => match r.expr.as_ref() {
            syn::Expr::Array(arr) => arr,
            _ => return Err("Not an array".into()),
        },
        _ => return Err("Not a reference".into()),
    };
    arr.elems
        .iter()
        .map(|x| parse_num::<u64>(x).map(Some))
        .collect()
}
