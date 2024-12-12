use glob::{glob_with, MatchOptions};
use quote::ToTokens;
use std::{
    collections::HashSet,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use syn::{parse::Parser, punctuated::Punctuated, Expr, ExprCall, ExprMacro, Token};

#[macro_use]
mod macros;
mods!(
    layout,
    flexbox_grid,
    spacing,
    sizing,
    typography,
    backgrounds,
    borders,
    effects,
    filters,
    tables,
    transitions_animation,
    transforms,
    interactivity,
    svg,
    accessibility
);

struct Visitor {
    instances: HashSet<String>,
    target_types: Vec<String>,
}

impl Visitor {
    fn new(target_types: Vec<String>) -> Self {
        Self {
            instances: HashSet::new(),
            target_types,
        }
    }

    fn is_target_type(&self, type_str: &str) -> bool {
        self.target_types.iter().any(|t| type_str.contains(t))
    }

    fn visit_file(&mut self, file_path: &Path) -> syn::Result<()> {
        let content = std::fs::read_to_string(file_path).unwrap();

        let ast = syn::parse_file(&content)?;
        syn::visit::visit_file(self, &ast);

        Ok(())
    }
}

impl<'ast> syn::visit::Visit<'ast> for Visitor {
    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        match i {
            Expr::Macro(ExprMacro { mac, .. }) => {
                let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
                let args = parser.parse2(mac.tokens.clone()).unwrap_or_default();

                args.iter().for_each(|arg| {
                    let arg_str = arg.to_token_stream().to_string();
                    if self.is_target_type(&arg_str) {
                        self.instances.insert(arg_str);
                    }
                    self.visit_expr(arg);
                });
            }
            Expr::Call(ExprCall { func, args, .. }) => {
                let func_str = func.to_token_stream().to_string();
                if self.is_target_type(&func_str) {
                    self.instances.insert(i.to_token_stream().to_string());
                }
                args.iter().for_each(|a| {
                    self.visit_expr(a);
                });
            }
            Expr::Path(expr_path) => {
                let path_str = expr_path.to_token_stream().to_string();
                if self.is_target_type(&path_str) && path_str.contains("::") {
                    self.instances.insert(i.to_token_stream().to_string());
                }
            }
            _ => {}
        }
        syn::visit::visit_expr(self, i);
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
    let target_types = types();
    let mut visitor = Visitor::new(target_types);

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
