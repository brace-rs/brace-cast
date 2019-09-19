extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::parse::Input;

mod parse;
mod tagged_impl;
mod tagged_trait;

#[proc_macro_attribute]
pub fn cast(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    TokenStream::from(match input {
        Input::Impl(input) => tagged_impl::expand(input),
        Input::Trait(input) => tagged_trait::expand(input),
    })
}
