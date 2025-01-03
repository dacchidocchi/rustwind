use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

pub fn derive_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let enum_data = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return Error::new_spanned(&input.ident, "Parse can only be used with enums")
                .to_compile_error()
                .into();
        }
    };

    let variant_matches = enum_data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string();
        quote! {
            #variant_str => Ok(Self::#variant_name),
        }
    });

    quote! {
        impl syn::parse::Parse for #enum_name {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let path: syn::Path = input.parse()?;
                let variant = path.segments.last().unwrap().ident.to_string();
                match variant.as_str() {
                    #(#variant_matches)*
                    _ => Err(syn::Error::new(input.span(), "Unknown variant")),
                }
            }
        }
    }
    .into()
}
