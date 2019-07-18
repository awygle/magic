use magic_types::*;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use std::fs::File;
use std::io::prelude::*;
use syn;

#[proc_macro]
pub fn forall_instructions(code: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(code as syn::LitStr);
    println!(
        "opening {} from {:?}",
        input.value(),
        std::env::current_dir().unwrap()
    );
    let mut f = File::open(&input.value()).unwrap();
    let mut src = String::new();
    f.read_to_string(&mut src).unwrap();
    let instrs: Vec<MetaInstruction> = serde_json::from_str(&src).unwrap();
    let names = instrs
        .into_iter()
        .map(|x| syn::Ident::new(&x.name.replace(".", "_"), proc_macro2::Span::call_site()));
    let result = quote! {
        #(
        enum #names {}
        )*
    };
    result.into()
}
