extern crate proc_macro;

use proc_macro::TokenStream;

use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::{Expr, ItemFn, LitStr, parse_macro_input};

/// print_start_end
#[proc_macro_attribute]
pub fn print_start_end(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attr = parse_macro_input!(attr as LitStr);
    let name = &input.sig.ident;
    let block = &input.block;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let guard_struct_ident = format_ident!("{}Guard", name.to_string().to_case(Case::Camel));
    let result = quote! {
        struct #guard_struct_ident;
        impl Drop for #guard_struct_ident {
            fn drop(&mut self) {
                println!("[end]====================================================[{}]",#attr);
                println!();
            }
        }
        pub fn #name(#inputs) #output {
            // println!();
            println!("[start]==================================================[{}]",#attr);
            let _guard = #guard_struct_ident;
            #block
        }
    };
    result.into()
}


#[proc_macro]
pub fn inner_print(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let expanded = quote! {
        println!("[inner_print]============================================[{}]",#expr);
    };

    TokenStream::from(expanded)
}

#[test]
pub fn test() {}
