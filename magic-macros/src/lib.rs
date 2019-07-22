use magic_types::*;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use std::fs::File;
use std::io::prelude::*;
use syn;

#[proc_macro]
pub fn vr4300_instr_enum(code: TokenStream) -> TokenStream {
    let enum_name = syn::parse_macro_input!(code as syn::Ident);
    let input = "../magic-macros/mipsiii.json";
    println!(
        "opening {} from {:?}",
        input,
        std::env::current_dir().unwrap()
    );
    let mut f = File::open(input).unwrap();
    let mut src = String::new();
    f.read_to_string(&mut src).unwrap();
    let instrs: Vec<MetaInstruction> = serde_json::from_str(&src).unwrap();
    let names = instrs
        .into_iter()
        .map(|x| syn::Ident::new(&x.name.replace(".", "_"), proc_macro2::Span::call_site()));
    let result = quote! {
        use strum_macros::EnumString;
        #[derive(Debug, Copy, Clone, PartialEq, EnumString)]
        pub enum #enum_name {
            Invalid,
        #(
            #names
        ),*
        }
    };
    result.into()
}

