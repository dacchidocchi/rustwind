use proc_macro::TokenStream;

mod as_class;
mod parse;

#[proc_macro_derive(Parse)]
pub fn parse_derive(input: TokenStream) -> TokenStream {
    parse::derive_parse(input)
}

#[proc_macro_derive(AsClass, attributes(as_class))]
pub fn as_class_derive(input: TokenStream) -> TokenStream {
    as_class::as_class_derive(input)
}
