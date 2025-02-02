use proc_macro::TokenStream;

mod display;
mod parse;

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    parse::derive_parse_impl(input)
}

#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display(input: TokenStream) -> TokenStream {
    display::derive_display_impl(input)
}

#[proc_macro_derive(ConstDisplay, attributes(display))]
pub fn derive_const_display(input: TokenStream) -> TokenStream {
    display::derive_const_display_impl(input)
}
