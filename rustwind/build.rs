use rustwind_codegen::{proc_macro2, syn, Codegen};
use std::{env, error::Error, path::PathBuf};

const TAILWIND_TYPES_PATH: &str = "../types.min.json";
const TYPES_OUTPUT_FILE: &str = "types.rs";
const UTILS_OUTPUT_FILE: &str = "utils.rs";

fn write_to_file(
    tokens: proc_macro2::TokenStream,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    let syntax_tree = syn::parse2(tokens)?;
    let formatted = prettyplease::unparse(&syntax_tree);

    let out_dir = PathBuf::from(env::var("OUT_DIR")?).join(output_file);
    std::fs::write(out_dir, formatted)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let codegen = Codegen::new(TAILWIND_TYPES_PATH)?;
    let modules = codegen.generate_mods();
    let utils = codegen.generate_utils();

    write_to_file(modules, TYPES_OUTPUT_FILE)?;
    write_to_file(utils, UTILS_OUTPUT_FILE)?;

    println!("cargo:rerun-if-changed={TAILWIND_TYPES_PATH}");
    Ok(())
}
