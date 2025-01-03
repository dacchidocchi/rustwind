use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Result, Write};
use tempfile::NamedTempFile;
use typewind::build;

fn process_content(content: &str) -> Result<HashSet<String>> {
    let input_temp_file = NamedTempFile::new()?;
    let input_path = input_temp_file.path();

    let mut input_file = input_temp_file.reopen()?;
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

    Ok(output_content
        .trim()
        .split_whitespace()
        .map(String::from)
        .collect())
}

#[test]
fn test_simple_path_expressions() -> Result<()> {
    let content = r#"
        fn main() {
            FontFamily::Sans;
            FontFamily::Mono;
            FontWeight::Bold;
        }
    "#;

    let classes = process_content(content)?;
    let expected = ["font-sans", "font-mono", "font-bold"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    assert_eq!(classes, expected);
    Ok(())
}

#[test]
fn test_method_calls() -> Result<()> {
    let content = r#"
        fn main() {
            div().class(BackgroundColor::Blue300);
            div()
                .class(Display::Flex)
                .children(
                    h1().class(FontFamily::Mono)
                );
        }
    "#;

    let classes = process_content(content)?;
    let expected = ["bg-blue-300", "flex", "font-mono"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    assert_eq!(classes, expected);
    Ok(())
}

#[test]
fn test_macro_expressions() -> Result<()> {
    let content = r#"
        fn main() {
            div().class(format!("{} {}", Display::Flex, FlexDirection::Col));
            div().class(tw!(Padding::P4, Gap::_4, hover!(BackgroundColor::Gray200)));
        }
    "#;

    let classes = process_content(content)?;
    let expected = ["flex", "flex-col", "p-4", "gap-4", "hover:bg-gray-200"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    assert_eq!(classes, expected);
    Ok(())
}

#[test]
fn test_match_expressions() -> Result<()> {
    let content = r#"
        fn main() {
            let layout = true;
            match layout {
                true => {
                    div()
                        .class(tw!(Display::Grid, Gap::_4))
                        .children(
                            section()
                                .class(tw!(Padding::P4, BorderRadius::Lg))
                        )
                }
                false => {
                    div().class(Display::Hidden)
                }
            };
        }
    "#;

    let classes = process_content(content)?;
    let expected = ["grid", "gap-4", "p-4", "rounded-lg", "hidden"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    assert_eq!(classes, expected);
    Ok(())
}

#[test]
fn test_tuple_expressions() -> Result<()> {
    let content = r#"
        fn main() {
            let styles = (BorderRadius::Md, Padding::P6);
            div().class(tw!(styles.0, styles.1));
        }
    "#;

    let classes = process_content(content)?;
    let expected = ["rounded-md", "p-6"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    assert_eq!(classes, expected);
    Ok(())
}

#[test]
fn test_block_expressions() -> Result<()> {
    let content = r#"
        fn main() {
            {
                div().class(Position::Relative);
                span().class(TextAlign::Center);
            }
        }
    "#;

    let classes = process_content(content)?;
    let expected = ["relative", "text-center"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    assert_eq!(classes, expected);
    Ok(())
}

#[test]
fn test_function_calls() -> Result<()> {
    let content = r#"
        fn some_function(a: View, b: View) -> View {
            div().children((a, b))
        }

        fn main() {
            some_function(
                div().class(Width::Full),
                span().class(Height::Screen)
            );
        }
    "#;

    let classes = process_content(content)?;
    let expected = ["w-full", "h-screen"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    assert_eq!(classes, expected);
    Ok(())
}

#[test]
fn test_complex_nested_expressions() -> Result<()> {
    let content = r#"
        enum Theme { Light, Dark }
        fn get_theme() -> Theme { Theme::Light }

        fn main() {
            match get_theme() {
                Theme::Light => div()
                    .class(tw!(
                        BackgroundColor::White,
                        TextColor::Gray900
                    ))
                    .children((
                        h1().class(FontSize::_2xl),
                        p().class(LineHeight::Relaxed)
                    )),
                Theme::Dark => {
                    let base_styles = format!("{} {}", 
                        BackgroundColor::Gray900,
                        TextColor::White
                    );
                    div()
                        .class(base_styles)
                        .children(
                            nav()
                                .class(tw!(Display::Flex, Gap::_4))
                        )
                }
            };
        }
    "#;

    let classes = process_content(content)?;
    let expected = [
        "bg-white",
        "text-gray-900",
        "text-2xl",
        "leading-relaxed",
        "bg-gray-900",
        "text-white",
        "flex",
        "gap-4",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    assert_eq!(classes, expected);
    Ok(())
}
