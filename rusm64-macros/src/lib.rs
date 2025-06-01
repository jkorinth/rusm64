use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EqModAddressing)]
pub fn derive_equal_mod_addressing(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Generate the AlmostEqual impl assuming the type implements Eq
    let expanded = quote! {
        impl EqModAddressing for #name {
            fn eq_mod_addressing(&self, other: &Self) -> bool {
                self == other
            }
        }
    };

    TokenStream::from(expanded)
}
