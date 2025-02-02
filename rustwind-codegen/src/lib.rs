use quote::{format_ident, quote};
use types::Section;

mod types;
mod utils;

pub use proc_macro2;
pub use syn;

pub struct Codegen {
    sections: Vec<Section>,
}

impl Codegen {
    pub fn new(types_path: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            sections: {
                let json_content = std::fs::read_to_string(types_path)?;
                serde_json::from_str::<Vec<Section>>(&json_content)?
            },
        })
    }

    pub fn generate_mods(&self) -> proc_macro2::TokenStream {
        let modules = self.sections.iter().map(|section| section.to_module());

        quote! {
            #(#modules)*
        }
    }

    pub fn generate_utils(&self) -> proc_macro2::TokenStream {
        let to_classes_fn = self.generate_to_classes_fn();
        let types_fn = self.generate_types_fn();

        quote! {
            mod utils {
                #to_classes_fn
                #types_fn
            }
        }
    }

    fn generate_to_classes_fn(&self) -> proc_macro2::TokenStream {
        let match_arms = self.sections.iter().flat_map(|section| {
            let module = format_ident!("{}", section.title_as_mod_name());
            section.categories.iter().map(move |category| {
                let type_ident = format_ident!("{}", category.title_as_enum_name());
                let type_string = category.title_as_enum_name();
                quote! {
                    #type_string => parse_and_format::<crate::#module::#type_ident>(state, expr_str),
                }
            })
        });

        quote! {
            pub(crate) fn to_classes(instances: &[crate::Instance]) -> Vec<String> {
                instances.iter()
                    .filter_map(|(state, expr_str)| parse_instance(state, expr_str))
                    .collect()
            }

            fn parse_instance(state: &Option<String>, expr_str: &str) -> Option<String> {
                match expr_str.split_whitespace().next()? {
                    #(#match_arms)*
                    _ => None,
                }
            }

            fn parse_and_format<T: syn::parse::Parse + std::fmt::Display>(
                state: &Option<String>,
                expr_str: &str
            ) -> Option<String> {
                syn::parse_str::<T>(expr_str)
                    .ok()
                    .map(|expr| {
                        let state_string = state.as_ref()
                            .map(|s| format!("{}:", s))
                            .unwrap_or_default();
                        format!("{}{}", state_string, expr)
                    })
            }
        }
    }

    fn generate_types_fn(&self) -> proc_macro2::TokenStream {
        let type_names = self
            .sections
            .iter()
            .flat_map(|section| &section.categories)
            .map(|category| category.title_as_enum_name())
            .collect::<Vec<_>>();

        let len = type_names.len();

        quote! {
            pub(crate) fn types() -> [&'static str; #len] {
                [#(#type_names),*]
            }
        }
    }
}
