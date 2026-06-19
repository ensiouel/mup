#![forbid(unsafe_code)]

use proc_macro::TokenStream;

mod component;
mod cursor;
mod markup;
mod mup_path;

#[proc_macro]
pub fn markup(input: TokenStream) -> TokenStream {
    markup::expand(input.into()).into()
}

#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    component::expand(input.into()).into()
}
