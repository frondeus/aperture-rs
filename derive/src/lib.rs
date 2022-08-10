extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod lenses;
mod prisms;

#[proc_macro_derive(Prism)]
pub fn prism_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let res = prisms::prism_derive(input);
    dbg!(&res.to_string());
    TokenStream::from(res)
}

#[proc_macro_derive(Lens)]
pub fn lens_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(lenses::lens_derive(input))
}
