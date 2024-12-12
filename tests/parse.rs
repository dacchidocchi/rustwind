use syn::parse_str;
use typewind::typography::FontFamily;

#[test]
fn test_font_family_parse_full_path() {
    let input = "typewind::typography::FontFamily::Mono";
    let result = parse_str::<FontFamily>(input);

    assert!(
        result.is_ok(),
        "Failed to parse FontFamily with full path: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap(), FontFamily::Mono);
}

#[test]
fn test_font_family_parse_typography_path() {
    let input = "typography::FontFamily::Sans";
    let result = parse_str::<FontFamily>(input);

    assert!(
        result.is_ok(),
        "Failed to parse FontFamily with typography path: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap(), FontFamily::Sans);
}

#[test]
fn test_font_family_parse_short_path() {
    let input = "FontFamily::Sans";
    let result = parse_str::<FontFamily>(input);

    assert!(
        result.is_ok(),
        "Failed to parse FontFamily with short path: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap(), FontFamily::Sans);
}

#[test]
fn test_font_family_variant_parse() {
    let input = "Sans";
    let result = parse_str::<FontFamily>(input);

    assert!(
        result.is_ok(),
        "Failed to parse FontFamily Sans type: {:?}",
        result.err()
    );
    assert_eq!(result.unwrap(), FontFamily::Sans);
}

#[test]
fn test_font_family_to_class() {
    let input = ["FontFamily::Sans", "FontFamily::Serif", "FontFamily::Mono"];
    let expected = ["font-sans", "font-serif", "font-mono"];

    let result = input
        .into_iter()
        .filter_map(|s| parse_str::<FontFamily>(s).ok())
        .map(|f| f.to_string())
        .collect::<Vec<String>>();

    assert_eq!(result, expected, "FontFamily to class conversion failed");
}
