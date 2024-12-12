use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Result, Write};
use tempfile::NamedTempFile;
use typewind::build;

#[test]
fn test_build() -> Result<()> {
    let input_temp_file = NamedTempFile::new()?;
    let input_path = input_temp_file.path();

    let mut input_file = input_temp_file.reopen()?;
    let content = r#"
        fn main() {
            FontFamily::Sans;
            div().class(BackgroundColor::Blue300);
            div().class(format!("{}", Padding::P10));
            div().class(format!("{} {}", Display::Flex, FlexDirection::Col));
        }
    "#;
    input_file.write_all(content.as_bytes())?;

    let output_temp_file = NamedTempFile::new()?;
    let output_path = output_temp_file.path();
    build(
        output_path.to_str().unwrap(),
        &[input_path.to_str().unwrap()],
    )?;

    let mut output_file = File::open(output_path)?;
    let mut output_content = String::new();
    output_file.read_to_string(&mut output_content)?;

    let classes = output_content
        .trim()
        .split_whitespace()
        .collect::<HashSet<&str>>();

    let expected_classes = ["font-sans", "bg-blue-300", "p-10", "flex", "flex-col"]
        .into_iter()
        .collect();

    assert_eq!(classes, expected_classes);

    Ok(())
}
