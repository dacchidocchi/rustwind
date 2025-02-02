use darling::FromVariant;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Ident};

#[derive(FromVariant)]
#[darling(attributes(display))]
struct VariantArgs {
    format: syn::LitStr,
}

fn generate_display_impl(input: DeriveInput, const_format: bool) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;

    let enum_data = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return Error::new_spanned(&input.ident, "Display can only be used with enums")
                .to_compile_error();
        }
    };

    let variant_matches = enum_data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let args = match VariantArgs::from_variant(variant) {
            Ok(args) => args,
            Err(err) => return err.write_errors(),
        };

        let format_value = args.format.value();

        match &variant.fields {
            Fields::Unit => match const_format {
                true => {
                    quote! { Self::#variant_name => ::const_format::writec!(f, #format_value), }
                }
                false => quote! { Self::#variant_name => write!(f, #format_value), },
            },
            Fields::Unnamed(fields) => {
                let field_count = fields.unnamed.len();
                let vars: Vec<Ident> = (0..field_count)
                    .map(|i| quote::format_ident!("val{}", i))
                    .collect();
                let pattern = &vars;
                let write_args = &vars;

                match const_format {
                    true => {
                        quote! {
                            Self::#variant_name(#(#pattern),*) =>
                                ::const_format::writec!(f, #format_value, #(#write_args),*),
                        }
                    }
                    false => {
                        quote! {
                            Self::#variant_name(#(#pattern),*) =>
                                write!(f, #format_value, #(#write_args),*),
                        }
                    }
                }
            }
            _ => Error::new_spanned(variant, "Only unit and tuple variants are supported")
                .to_compile_error(),
        }
    });

    match const_format {
        true => quote! {
            impl ::const_format::pmr::FormatMarker for #enum_name {
                type Kind = ::const_format::pmr::IsNotStdKind;
                type This = Self;
            }

            impl #enum_name {
                pub const fn const_display_fmt(
                    &self,
                    f: &mut ::const_format::pmr::Formatter<'_>,
                ) -> ::const_format::pmr::Result<(), ::const_format::pmr::Error> {
                    match self {
                        #(#variant_matches)*
                    }
                }
            }
        },
        false => quote! {
            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#variant_matches)*
                    }
                }
            }
        },
    }
}

pub fn derive_display_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    generate_display_impl(input, false).into()
}

pub fn derive_const_display_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    generate_display_impl(input, true).into()
}
