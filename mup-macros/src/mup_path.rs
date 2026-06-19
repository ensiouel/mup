use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn get() -> TokenStream {
    match crate_name("mup") {
        Ok(FoundCrate::Itself) => quote!(::mup),
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name.replace('-', "_"), Span::call_site());
            quote!(::#ident)
        }
        Err(_) => quote!(::mup),
    }
}
