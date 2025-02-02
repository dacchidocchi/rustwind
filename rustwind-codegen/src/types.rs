use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use syn::Ident;

use crate::utils::{StringExt, VecStringExt};

/// Represents a section of the Tailwind documentation.
#[derive(Deserialize)]
pub struct Section {
    /// The title of the section.
    /// e.g. "Layout"
    title: String,
    /// The categories within the section.
    /// e.g. [Category("Aspect Ratio", "...", [Variant(name="aspect-auto", properties="aspect-ratio: auto;")])]
    pub categories: Vec<Category>,
}

impl Section {
    pub fn to_module(&self) -> TokenStream {
        let mod_ident = format_ident!("{}", self.title_as_mod_name());

        let enums = self.categories.iter().map(|c| {
            let class_prefix = c
                .variants
                .iter()
                .map(|v| v.class.clone())
                // For the moment, ignore classes that start with a "-".
                // TODO: add support for negative classes adding some method like .neg() to the enum?
                .filter(|class| !class.starts_with("-"))
                .collect::<Vec<_>>()
                .find_common_prefix()
                .unwrap_or_default();

            c.to_enum(&class_prefix)
        });

        quote! {
            pub mod #mod_ident {
                use rustwind_macros::{ConstDisplay, Display, Parse};

                #(#enums)*
            }
        }
    }

    pub fn title_as_mod_name(&self) -> String {
        self.title
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .to_case(Case::Snake)
    }
}

/// Represents a category within a section of the Tailwind documentation.
#[derive(Deserialize)]
pub struct Category {
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
        let enum_name = self.title_as_enum_name();
        let enum_ident = format_ident!("{}", enum_name);

        let description_doc = format!(" {}", self.description);
        let url_doc = format!(" <{}>", self.url);

        let variants = self
            .variants
            .iter()
            .filter_map(|v| v.to_enum_variant(class_prefix));

        quote! {
            #[doc = #description_doc]
            #[doc = ""]
            #[doc = #url_doc]
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, ConstDisplay, Display, Parse)]
            pub enum #enum_ident {
                #(#variants),*
            }
        }
    }

    pub fn title_as_enum_name(&self) -> String {
        self.title
            .replace(|c: char| !c.is_alphanumeric(), "-")
            .to_case(Case::Pascal)
    }
}

/// Represents a variant within a category of the Tailwind documentation.
#[derive(Deserialize)]
pub struct Variant {
    /// The name of the variant.
    /// e.g. "aspect-auto"
    class: String,
    /// The properties of the variant.
    /// e.g. "aspect-ratio: auto;"
    properties: String,
}

impl Variant {
    fn to_enum_variant(&self, class_prefix: &str) -> Option<TokenStream> {
        if self.class.starts_with('-') {
            return None;
        }

        let variant_ident = self.variant_ident(class_prefix);
        let variant_type = self.determine_variant_type();
        let variant_doc = self.format_properties_doc();

        let format = self.class.replace_between_delimiters_with_indices('<', '>');
        let format = match self.class.contains("<fraction>") {
            true => format.replace("{0}", "{0}/{1}"),
            false => format,
        };

        match variant_type {
            Some(variant_type) => Some(quote! {
                #[doc = #variant_doc]
                #[display(format = #format)]
                #variant_ident(#variant_type)
            }),
            None => Some(quote! {
                #[doc = #variant_doc]
                #[display(format = #format)]
                #variant_ident
            }),
        }
    }

    fn variant_ident(&self, class_prefix: &str) -> Ident {
        let sanitized_class = self
            .class
            .chars()
            .map(|c| match c.is_alphanumeric() {
                true => c,
                false => '-',
            })
            .collect::<String>();

        let variant_name = match sanitized_class == class_prefix {
            true => sanitized_class.as_str(),
            false => sanitized_class
                .strip_prefix(class_prefix)
                .map(|s| s.trim_start_matches('-'))
                .unwrap_or(&self.class),
        }
        .to_case(Case::Pascal)
        .prepend_underscore();

        format_ident!("{}", variant_name)
    }

    fn determine_variant_type(&self) -> Option<TokenStream> {
        if !self.class.contains('<') {
            return None;
        }

        let types = self
            .class
            .extract_delimited_values('<', '>')
            .into_iter()
            .map(|v| match v.as_str() {
                "fraction" => quote!(u8, u8),
                _ => quote!(&'static str),
            })
            .collect::<Vec<_>>();

        match types.len() {
            0 => None,
            1 => Some(types[0].clone()),
            _ => Some(quote!(#(#types),*)),
        }
    }

    fn format_properties_doc(&self) -> String {
        format!(
            "```css\n{{\n{}\n}}\n```",
            self.properties
                .lines()
                .map(|l| format!("    {}", l))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
