use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro]
pub fn gen_square_consts(_input: TokenStream) -> TokenStream {
    let lines = (0..64).map(|i| {
        let ident = format_ident!("{}{}", (b'A' + i % 8) as char, i / 8 + 1);
        let value: u64 = 1u64 << i;
        quote! { const #ident: u64 = #value; }
    });
    quote! {
        #(#lines)*
    }
    .into()
}
