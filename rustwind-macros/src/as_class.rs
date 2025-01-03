use darling::FromVariant;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

#[derive(FromVariant)]
#[darling(attributes(as_class))]
struct VariantArgs {
    class: syn::LitStr,
}

pub fn as_class_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let enum_data = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return Error::new_spanned(input.ident, "AsClass can only be used with enums")
                .to_compile_error()
                .into();
        }
    };

    let variant_matches = enum_data.variants.iter().map(|variant| {
        let args = match VariantArgs::from_variant(variant) {
            Ok(args) => args,
            Err(err) => return err.write_errors(),
        };

        let variant_name = &variant.ident;
        let class_value = args.class.value();
        quote! {
            Self::#variant_name => #class_value,
        }
    });

    quote! {
        impl #enum_name {
            pub const fn as_class(&self) -> &'static str {
                match self {
                    #(#variant_matches)*
                }
            }
        }
    }
    .into()
}
