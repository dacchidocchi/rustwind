use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

const TAILWIND_TYPES_PATH: &str = "../types.min.json";
const TYPES_OUTPUT_FILE: &str = "types.rs";
const UTILS_OUTPUT_FILE: &str = "utils.rs";

pub trait StringExt {
    /// Returns a new string containing only alphanumeric characters.
    fn alphanumeric_only(&self) -> String;
}

impl StringExt for String {
    fn alphanumeric_only(&self) -> String {
        self.chars().filter(|c| c.is_alphanumeric()).collect()
    }
}

/// Finds the common prefix of a list of strings.
fn find_common_prefix(strings: &[String]) -> Option<String> {
    if strings.len() <= 1 {
        return None;
    }

    let parts_vec: Vec<Vec<&str>> = strings.iter().map(|s| s.split('-').collect()).collect();

    let common_parts = parts_vec[0]
        .iter()
        .enumerate()
        .take_while(|(i, &current_part)| {
            parts_vec[1..]
                .iter()
                .all(|parts| parts.get(*i).map_or(false, |&part| part == current_part))
        })
        .map(|(_, &part)| part)
        .collect::<Vec<_>>();

    match common_parts.is_empty() {
        true => None,
        false => Some(common_parts.join("-")),
    }
}

/// Represents a section of the Tailwind documentation.
#[derive(Deserialize)]
struct Section {
    /// The title of the section.
    /// e.g. "Layout"
    title: String,
    /// The categories within the section.
    /// e.g. [Category("Aspect Ratio", "...", [Variant(name="aspect-auto", properties="aspect-ratio: auto;")])]
    categories: Vec<Category>,
}

impl Section {
    fn to_module(&self) -> TokenStream {
        let mod_name = format_ident!("{}", self.title.alphanumeric_only().to_case(Case::Snake));

        let enums = self
            .categories
            .iter()
            .map(|c| {
                let class_prefix = find_common_prefix(
                    &c.variants
                        .iter()
                        .map(|v| v.class.clone())
                        .collect::<Vec<_>>(),
                )
                .unwrap_or_default();

                c.to_enum(&class_prefix)
            })
            .collect::<Vec<_>>();

        quote! {
            pub mod #mod_name {
                use rustwind_macros::{AsClass, Parse};

                #(#enums)*
            }
        }
    }
}

/// Represents a category within a section of the Tailwind documentation.
#[derive(Deserialize)]
struct Category {
    /// The URL of the category on the Tailwind documentation website.
    /// e.g. "https://tailwindcss.com/docs/aspect-ratio"
    url: String,
    /// The title of the category.
    /// e.g. "Aspect Ratio"
    title: String,
    /// The description of the category.
    /// e.g. "Utilities for controlling the aspect ratio of an element."
    description: String,
    /// The variants within the category.
    /// e.g. [Variant(name="aspect-auto", properties="aspect-ratio: auto;")]
    variants: Vec<Variant>,
}

impl Category {
    fn to_enum(&self, class_prefix: &str) -> TokenStream {
        let enum_name = format_ident!("{}", self.title.alphanumeric_only().to_case(Case::Pascal));
        let description_doc = format!(" {}", self.description);
        let url_doc = format!(" <{}>", self.url);

        let variants = self
            .variants
            .iter()
            .map(|v| v.to_enum_variant(class_prefix))
            .collect::<Vec<_>>();

        quote! {
            #[doc = #description_doc]
            #[doc = ""]
            #[doc = #url_doc]
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, AsClass, Parse)]
            pub enum #enum_name {
                #(#variants),*
            }
        }
    }
}

/// Represents a variant within a category of the Tailwind documentation.
#[derive(Deserialize)]
struct Variant {
    /// The name of the variant.
    /// e.g. "aspect-auto"
    class: String,
    /// The properties of the variant.
    /// e.g. "aspect-ratio: auto;"
    properties: String,
}

impl Variant {
    fn clean_class_name(&self, common_prefix: &str) -> String {
        let base_name = match self.class == common_prefix {
            true => self.class.as_str(),
            false => self
                .class
                .strip_prefix(common_prefix)
                .map(|s| s.trim_start_matches('-'))
                .unwrap_or(&self.class),
        };

        let sanitized_name = base_name
            .replace(".", "_")
            .replace("/", "over")
            .replace("%", "percent");

        let transformed_name = sanitized_name
            .split('-')
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first_char) => {
                        first_char.to_uppercase().collect::<String>() + chars.as_str()
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("");

        match transformed_name
            .chars()
            .next()
            .map_or(false, |c| c.is_numeric())
        {
            true => format!("_{}", transformed_name),
            false => transformed_name,
        }
    }

    fn to_enum_variant(&self, class_prefix: &str) -> TokenStream {
        let variant_name = format_ident!("{}", self.clean_class_name(class_prefix));
        let class_literal = &self.class;
        let properties_doc = format!(
            "```css\n{{\n{}\n}}\n```",
            self.properties
                .lines()
                .map(|l| format!("    {}", l))
                .collect::<Vec<_>>()
                .join("\n")
        );

        quote! {
            #[doc = #properties_doc]
            #[as_class(class = #class_literal)]
            #variant_name
        }
    }
}

/// Generates a function that converts a slice of `Instance`s into a vector of final Tailwind classes.
fn generate_to_classes_fn(types: &[(String, String)]) -> TokenStream {
    let module_idents = types.iter().map(|(m, _)| format_ident!("{m}"));
    let type_idents = types.iter().map(|(_, t)| format_ident!("{t}"));
    let type_strings = types.iter().map(|(_, t)| t);

    quote! {
        fn to_classes(instances: &[crate::Instance]) -> Vec<String> {
            instances.iter()
                .filter_map(|(state, expr_str)| {
                    match expr_str.split_whitespace().next()? {
                        #(
                            #type_strings => syn::parse_str::<crate::#module_idents::#type_idents>(expr_str)
                                .ok()
                                .map(|expr| {
                                    let state_str = state.as_ref()
                                        .map(|s| format!("{}:", s))
                                        .unwrap_or_default();
                                    format!("{}{}", state_str, expr.as_class())
                                }),
                        )*
                        _ => None,
                    }
                })
                .collect()
        }
    }
}

/// Generates a function that returns a slice of the all Tailwind types.
fn generate_types_fn(type_names: &Vec<String>) -> TokenStream {
    let len = type_names.len();

    quote! {
        fn types() -> [&'static str; #len] {
            [#(#type_names),*]
        }
    }
}

/// Writes the given `TokenStream` to a file.
fn write_to_file(tokens: TokenStream, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let syntax_tree = syn::parse2(tokens)?;
    let formatted = prettyplease::unparse(&syntax_tree);

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join(output_file);
    std::fs::write(out_dir, formatted)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_content = std::fs::read_to_string(TAILWIND_TYPES_PATH)?;
    let sections = serde_json::from_str::<Vec<Section>>(&json_content)?;

    let mut types = vec![];
    let mut modules = vec![];
    sections.into_iter().for_each(|section| {
        types.extend(section.categories.iter().map(|category| {
            (
                section.title.alphanumeric_only().to_case(Case::Snake),
                category.title.alphanumeric_only().to_case(Case::Pascal),
            )
        }));
        modules.push(section.to_module());
    });
    let modules_tokens = quote! {
        #(#modules)*
    };
    write_to_file(modules_tokens, TYPES_OUTPUT_FILE)?;

    let to_classes_fn = generate_to_classes_fn(&types);
    let types_fn = generate_types_fn(&types.into_iter().map(|(_, t)| t).collect());
    let utils_tokens = quote! {
        #to_classes_fn
        #types_fn
    };
    write_to_file(utils_tokens, UTILS_OUTPUT_FILE)?;

    println!("cargo:rerun-if-changed={TAILWIND_TYPES_PATH}");
    Ok(())
}
