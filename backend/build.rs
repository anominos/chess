use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dst_path = Path::new(&out_dir).join("generated.rs");
    let mut tokens = TokenStream::new();
    tokens.extend(gen_square_consts());
    tokens.extend(gen_knight_moves());
    tokens.extend(gen_king_moves());
    tokens.extend(gen_bishop_moves());
    tokens.extend(gen_rook_moves());

    fs::write(dst_path, tokens.to_string()).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}

fn gen_square_consts() -> TokenStream {
    let lines = (0..64).map(|i| {
        let ident = format_ident!("{}{}", (b'A' + i % 8) as char, i / 8 + 1);
        let value: u64 = 1u64 << i;
        quote! {
            #[allow(dead_code)]
            pub const #ident: u64 = #value;
        }
    });
    quote! {
        #(#lines)*
    }
}

fn gen_knight_moves() -> TokenStream {
    let mut array = [0u64; 64];
    for knight in 0usize..64 {
        let row = (knight / 8) as i8;
        let col = (knight % 8) as i8;
        let mut board: u64 = 0;
        for dr in -2i8..=2 {
            if dr == 0 {
                continue;
            }
            for dc in [3 - dr.abs(), -3 + dr.abs()] {
                if (0..8).contains(&(row + dr)) && (0..8).contains(&(col + dc)) {
                    board |= 1 << ((row + dr) * 8 + col + dc);
                }
            }
        }
        array[knight] = board;
    }
    quote! {
        #[allow(dead_code)]
        pub const KNIGHT_MOVES: [u64; 64] = [#(#array),*];
    }
}

fn gen_king_moves() -> TokenStream {
    let mut array = [0u64; 64];
    for king in 0usize..64 {
        let row = (king / 8) as i8;
        let col = (king % 8) as i8;
        let mut board: u64 = 0;
        for dr in -1i8..=1 {
            for dc in -1i8..=1 {
                if (0..8).contains(&(row + dr)) && (0..8).contains(&(col + dc)) {
                    board |= 1 << ((row + dr) * 8 + col + dc)
                }
            }
        }
        array[king] = board & !(1u64 << king);
    }
    quote! {
        #[allow(dead_code)]
        pub const KING_MOVES: [u64; 64] = [#(#array),*];
    }
}

fn gen_bishop_moves() -> TokenStream {
    let mut array = [0u64; 64];
    for bishop in 0usize..64 {
        let row = (bishop / 8) as i8;
        let col = (bishop % 8) as i8;
        let mut board: u64 = 0;

        for (dr, dc) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
            let (mut r, mut c) = (row + dr, col + dc);
            while (0..8).contains(&r) && (0..8).contains(&c) {
                board |= 1u64 << (r * 8 + c);
                r += dr;
                c += dc;
            }
        }

        array[bishop] = board;
    }
    quote! {
        #[allow(dead_code)]
        pub const BISHOP_MOVES: [u64; 64] = [#(#array),*];
    }
}

fn gen_rook_moves() -> TokenStream {
    let mut array = [0u64; 64];
    for rook in 0usize..64 {
        let row = (rook / 8) as i8;
        let col = (rook % 8) as i8;
        let mut board: u64 = 0;

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (mut r, mut c) = (row + dr, col + dc);
            while (0..8).contains(&r) && (0..8).contains(&c) {
                board |= 1u64 << (r * 8 + c);
                r += dr;
                c += dc;
            }
        }

        array[rook] = board;
    }
    quote! {
        #[allow(dead_code)]
        pub const ROOK_MOVES: [u64; 64] = [#(#array),*];
    }
}
