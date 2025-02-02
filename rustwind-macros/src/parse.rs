use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Error};

pub fn derive_parse_impl(input: TokenStream) -> TokenStream {
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
        let variant_string = variant_name.to_string();

        match &variant.fields {
            syn::Fields::Unit => quote! {
                #variant_string => Ok(Self::#variant_name),
            },
            syn::Fields::Unnamed(fields_unnamed) => {
                let field_parsers = fields_unnamed.unnamed.iter().enumerate().map(|(i, field)| {
                    let lit_dent = quote::format_ident!("literal_{}", i);
                    let parsed_ident = quote::format_ident!("parsed_{}", i);
                    let field_type = field.ty.to_token_stream().to_string();

                    let parse_field = match field_type.as_str() {
                        "u8" => quote! {
                            let #lit_dent: syn::LitInt = content.parse()?;
                            let #parsed_ident = #lit_dent.base10_parse()?;
                        },
                        "& 'static str" => quote! {
                            let #lit_dent: syn::LitStr = content.parse()?;
                            let #parsed_ident = #lit_dent.value().leak();
                        },
                        _ => {
                            return Error::new_spanned(
                                field,
                                "Only u8 and &'static str fields are supported",
                            )
                            .to_compile_error()
                        }
                    };
                    match i < fields_unnamed.unnamed.len() - 1 {
                        true => quote! {
                            #parse_field
                            content.parse::<syn::Token![,]>()?;
                        },
                        false => quote! {
                            #parse_field
                        },
                    }
                });

                let field_vars =
                    (0..fields_unnamed.unnamed.len()).map(|i| quote::format_ident!("parsed_{}", i));

                quote! {
                    #variant_string => {
                        let content;
                        syn::parenthesized!(content in input);
                        #(#field_parsers)*
                        Ok(Self::#variant_name(#(#field_vars),*))
                    }
                }
            }
            _ => Error::new_spanned(variant, "Only unit and tuple variants are supported")
                .to_compile_error(),
        }
    });

    quote! {
        impl syn::parse::Parse for #enum_name {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let path: syn::Path = input.parse()?;
                let variant = path.segments.last().unwrap().ident.to_string();
                match variant.as_str() {
                    #(#variant_matches)*
                    _ => Err(syn::Error::new(input.span(), "Tried to parse an unknown variant")),
                }
            }
        }
    }
    .into()
}
