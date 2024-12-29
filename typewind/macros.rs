macro_rules! def_types {
    (
        $(
            $(#[$enum_meta:meta])*
            $name:ident {
                $(
                    $(#[$variant_meta:meta])*
                    $variant:ident => $val:expr
                ),+
                $(,)?
            }
        )+
    ) => {
        $(
            $(#[$enum_meta])*
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
            pub enum $name {
                $(
                    $(#[$variant_meta])*
                    $variant,
                )+
            }

            impl $name {
                pub const fn as_class(&self) -> &'static str {
                    match self {
                        $(
                            Self::$variant => $val,
                        )+
                    }
                }
            }

            impl std::fmt::Display for $name {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", match self {
                        $(
                            Self::$variant => $val,
                        )+
                    })
                }
            }

            impl syn::parse::Parse for $name {
                #[inline]
                fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                    let path: syn::Path = input.parse()?;
                    let variant = path.segments.last().unwrap().ident.to_string();
                    match variant.as_str() {
                        $(
                            stringify!($variant) => Ok(Self::$variant),
                        )+
                        _ => Err(syn::Error::new(input.span(), "Unknown variant")),
                    }
                }
            }
        )+

        pub(crate) fn types() -> Vec<String> {
            vec![$(stringify!($name).to_string()),*]
        }

        pub(crate) fn to_classes(instances: &[crate::Instance]) -> Vec<String> {
            instances.iter()
                .filter_map(|(state, expr_str)| {
                    match expr_str.split_whitespace().next()? {
                        $(
                            stringify!($name) => syn::parse_str::<$name>(expr_str)
                                .ok()
                                .map(|expr| {
                                    let state_str = state.as_ref().map(|s| format!("{}:", s)).unwrap_or_default();
                                    format!("{}{}", state_str, expr.as_class())
                                }),
                        )+
                        _ => None,
                    }
                })
                .collect()
        }
    };
}

macro_rules! mods {
    ($($mod_name:ident),*) => {
        $(pub mod $mod_name;)*

        fn types() -> Vec<String> {
            [$($mod_name::types()),*].concat()
        }

        fn to_classes(instances: &[crate::Instance]) -> Vec<String> {
            [$($mod_name::to_classes(instances)),*].concat()
        }
    };
}

macro_rules! def_states {
    ( $( $state:ident ),* ) => {
        $(
            #[macro_export]
            macro_rules! $state {
                ( $arg:path ) => {
                    $crate::const_format::concatcp!(stringify!($state), ":", ($arg).as_class())
                };
            }
        )*

        pub(crate) fn states() -> Vec<String> {
            vec![$(stringify!($state).to_string()),*]
        }
    };
}

def_states!(hover, focus, active);
