extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Lens)]
pub fn lens_derive(input: TokenStream) -> TokenStream {
    input
}
