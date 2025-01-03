use glob::{glob_with, MatchOptions};
use quote::ToTokens;
use std::{
    collections::HashSet,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use syn::{
    parse::Parser, punctuated::Punctuated, Expr, ExprBlock, ExprCall, ExprMacro, ExprMatch,
    ExprMethodCall, ExprTuple, Token,
};

pub use const_format;

include!(concat!(env!("OUT_DIR"), "/types.rs"));
include!(concat!(env!("OUT_DIR"), "/utils.rs"));

/// Counts the number of tokens in a macro invocation.
macro_rules! tt_count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + tt_count!($($tail)*) };
}

macro_rules! def_states {
    ($($state:ident),*) => {
        $( #[macro_export]
        macro_rules! $state {
            ($arg:path) => {
                $crate::const_format::concatcp!(stringify!($state), ":", ($arg).as_class())
            };
        })*

        fn states() -> [&'static str; tt_count!($($state)*)] {
            [$(stringify!($state)),*]
        }
    };
}

def_states!(hover, focus, active);

// State - Value
type Instance = (Option<String>, String);

struct Visitor<const T: usize, const S: usize> {
    instances: HashSet<Instance>,

    target_types: [&'static str; T],
    states: [&'static str; S],
}

impl<const T: usize, const S: usize> Visitor<T, S> {
    fn new(target_types: [&'static str; T], states: [&'static str; S]) -> Self {
        Self {
            instances: HashSet::new(),
            target_types,
            states,
        }
    }

    fn is_target_type(&self, type_str: &str) -> bool {
        self.target_types.iter().any(|t| type_str.contains(t))
    }

    fn is_state_macro(&self, macro_name: &str) -> bool {
        self.states.iter().any(|state| macro_name.contains(state))
    }

    fn visit_file(&mut self, file_path: &Path) -> syn::Result<()> {
        let content = std::fs::read_to_string(file_path).unwrap();

        let ast = syn::parse_file(&content)?;
        syn::visit::visit_file(self, &ast);

        Ok(())
    }
}

impl<'ast, const T: usize, const S: usize> syn::visit::Visit<'ast> for Visitor<T, S> {
    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        match i {
            Expr::Macro(ExprMacro { mac, .. }) => {
                let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
                let args = match parser.parse2(mac.tokens.clone()) {
                    Ok(parsed_args) => parsed_args,
                    Err(e) => {
                        eprintln!("Error parsing macro arguments: {}", e);
                        return;
                    }
                };

                let macro_name = mac
                    .path
                    .get_ident()
                    .expect("Expected identifier for macro")
                    .to_string();

                args.iter()
                    .for_each(|arg| match self.is_state_macro(&macro_name) {
                        true => {
                            let arg_string = arg.to_token_stream().to_string();
                            if self.is_target_type(&arg_string) {
                                self.instances
                                    .insert((Some(macro_name.clone()), arg_string));
                            }
                        }
                        false => {
                            self.visit_expr(arg);
                        }
                    });
            }
            Expr::Match(ExprMatch { arms, .. }) => {
                arms.iter().for_each(|arm| {
                    self.visit_pat(&arm.pat);
                    self.visit_expr(&arm.body);
                });
            }
            Expr::Block(ExprBlock { block, .. }) => {
                self.visit_block(block);
            }
            Expr::Tuple(ExprTuple { elems, .. }) | Expr::Call(ExprCall { args: elems, .. }) => {
                elems.iter().for_each(|a| {
                    self.visit_expr(a);
                });
            }
            Expr::MethodCall(ExprMethodCall { receiver, args, .. }) => {
                self.visit_expr(receiver);
                args.iter().for_each(|a| {
                    self.visit_expr(a);
                });
            }
            Expr::Path(expr_path) => {
                let path_string = expr_path.to_token_stream().to_string();
                if path_string.contains("::") && self.is_target_type(&path_string) {
                    self.instances
                        .insert((None, i.to_token_stream().to_string()));
                }
            }
            _ => {}
        }
    }
}

fn iter_files(patterns: &[&str]) -> Vec<PathBuf> {
    let match_options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut all_files: HashSet<PathBuf> = HashSet::new();
    let mut excluded_files: HashSet<PathBuf> = HashSet::new();

    patterns.iter().for_each(|pattern| {
        let is_negative = pattern.starts_with('!');
        let actual_pattern = pattern.strip_prefix('!').unwrap_or(pattern);

        if let Ok(entries) = glob_with(actual_pattern, match_options) {
            entries.filter_map(Result::ok).for_each(|entry| {
                match is_negative {
                    true => excluded_files.insert(entry),
                    false => all_files.insert(entry),
                };
            });
        }
    });

    all_files
        .difference(&excluded_files)
        .filter(|&path| path.is_file())
        .cloned()
        .collect()
}

/// Generates the final Tailwind classes from the Typewind types.
///
/// # Arguments
/// - `output_file` - A string slice that holds the path where the converted Tailwind classes will be saved.
///     This file must be linked to `tailwind.config.js` in order for Tailwind to generate the
///     necessary styles.
/// - `content` - A list of string slices containing paths to files that contain typewind types.
///     This is similar to the `content` field in Tailwind's configuration, supporting patterns like `./src/**/*.rs`,
///     and negated patterns like `!./src/lib.rs` to exclude files from the build.
///
/// # Example
/// ```no_run
/// // build.rs
/// use typewind::build;
///
#[allow(clippy::needless_doctest_main)]
/// fn main() {
///     build("../target/classes.txt", &["./src/**/*.rs"]).unwrap();
/// }
/// ```
/// ```js
/// // tailwind.config.js
/// module.exports = {
///     content: [
///         '../target/classes.txt',
///     ],
///     theme: {
///         extend: {},
///     },
///     plugins: [],
/// }
/// ```
pub fn build(output_file: &str, content: &[&str]) -> std::io::Result<()> {
    let mut visitor = Visitor::new(types(), states());

    iter_files(content).iter().for_each(|file_path| {
        if let Err(e) = visitor.visit_file(file_path) {
            eprintln!("Error processing file {:?}: {}", file_path, e);
        }
    });

    let instances = visitor.instances.into_iter().collect::<Vec<_>>();
    let classes = to_classes(&instances);

    let mut file = File::create(output_file)?;
    file.write_all(classes.join("\n").as_bytes())?;

    Ok(())
}
