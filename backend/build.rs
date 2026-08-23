use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dst_path = Path::new(&out_dir).join("generated.rs");
    let mut tokens = TokenStream::new();
    tokens.extend(gen_square_consts());
    tokens.extend(gen_knight_moves());

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
