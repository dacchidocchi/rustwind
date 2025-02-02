pub trait StringExt {
    /// Replaces content between specified delimiters with positional indices like
    /// `{0}`, `{1}`, `{2}`, etc.
    fn replace_between_delimiters_with_indices(&self, start_delim: char, end_delim: char)
        -> String;

    /// Extracts the values between specified delimiters.
    fn extract_delimited_values(&self, start_delim: char, end_delim: char) -> Vec<String>;

    /// Prepends an underscore if the string starts with a number.
    /// Returns the original string if it doesn't start with a number.
    fn prepend_underscore(&self) -> String;
}

impl StringExt for str {
    fn replace_between_delimiters_with_indices(
        &self,
        start_delim: char,
        end_delim: char,
    ) -> String {
        let mut result = String::with_capacity(self.len());
        let mut counter = 0;
        let mut inside_delimiters = false;

        self.chars().for_each(|ch| match (ch, inside_delimiters) {
            (c, false) if c == start_delim => {
                inside_delimiters = true;
            }
            (c, true) if c == end_delim => {
                inside_delimiters = false;
                result.push_str(&format!("{{{}}}", counter));
                counter += 1;
            }
            (_, true) => {}
            (c, _) => result.push(c),
        });

        result
    }

    fn extract_delimited_values(&self, start_delim: char, end_delim: char) -> Vec<String> {
        let mut inside_delimiters = false;
        let mut current_value = String::new();
        let mut result = Vec::new();

        self.chars().for_each(|ch| match (ch, inside_delimiters) {
            (c, false) if c == start_delim => {
                inside_delimiters = true;
            }
            (c, true) if c == end_delim => {
                inside_delimiters = false;
                if !current_value.is_empty() {
                    result.push(current_value.clone());
                    current_value.clear();
                }
            }
            (c, true) => {
                current_value.push(c);
            }
            _ => {}
        });

        result
    }

    fn prepend_underscore(&self) -> String {
        match self.chars().next().is_some_and(|c| c.is_numeric()) {
            true => format!("_{}", self),
            false => self.to_owned(),
        }
    }
}

pub trait VecStringExt {
    /// Finds the common prefix of a list of **kebab-cased** strings.
    fn find_common_prefix(&self) -> Option<String>;
}

impl VecStringExt for Vec<String> {
    fn find_common_prefix(&self) -> Option<String> {
        if self.is_empty() {
            return None;
        }

        self.iter()
            .skip(1)
            .try_fold(self[0].clone(), |mut prefix, s| {
                while !s.starts_with(&prefix) {
                    match prefix.rfind('-') {
                        Some(pos) => prefix.truncate(pos),
                        None => {
                            return None;
                        }
                    }
                }
                Some(prefix)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_between_delimiters_with_indices() {
        let input = "text-<size>/<number>/<color>";
        let expected_output = "text-{0}/{1}/{2}";

        let result = input.replace_between_delimiters_with_indices('<', '>');

        assert_eq!(
            result, expected_output,
            "The result does not match the expected output"
        );
    }

    #[test]
    fn test_no_delimiters() {
        let input = "text-size/number/color";
        let expected_output = "text-size/number/color";

        let result = input.replace_between_delimiters_with_indices('<', '>');

        assert_eq!(
            result, expected_output,
            "Text without delimiters should remain unchanged"
        );
    }

    #[test]
    fn test_multiple_replacements() {
        let input = "<first>-<second>-<third>";
        let expected_output = "{0}-{1}-{2}";

        let result = input.replace_between_delimiters_with_indices('<', '>');

        assert_eq!(
            result, expected_output,
            "Multiple delimiters should be replaced correctly"
        );
    }

    #[test]
    fn test_extract_single_value() {
        let input = "text-<size>/other";
        let values = input.extract_delimited_values('<', '>');
        assert_eq!(values, ["size"], "Should extract one delimited value");
    }

    #[test]
    fn test_extract_multiple_values() {
        let input = "<first>/<second>/<third>";
        let values = input.extract_delimited_values('<', '>');
        assert_eq!(
            values,
            ["first", "second", "third"],
            "Should extract three delimited values"
        );
    }

    #[test]
    fn test_extract_no_delimiters() {
        let input = "text-size/number/color";
        let values = input.extract_delimited_values('<', '>');
        assert!(
            values.is_empty(),
            "Should return empty vec when no delimiters are present"
        );
    }

    #[test]
    fn test_extract_with_different_delimiters() {
        let input = "[width]/(height)/{depth}";

        let square_values = input.extract_delimited_values('[', ']');
        let round_values = input.extract_delimited_values('(', ')');
        let curly_values = input.extract_delimited_values('{', '}');

        assert_eq!(
            square_values,
            ["width"],
            "Should extract one square bracket value"
        );
        assert_eq!(
            round_values,
            ["height"],
            "Should extract one parentheses value"
        );
        assert_eq!(
            curly_values,
            ["depth"],
            "Should extract one curly brace value"
        );
    }

    #[test]
    fn test_extract_empty_delimiters() {
        let input = "text<>between<>delimiters";
        let values = input.extract_delimited_values('<', '>');
        assert!(
            values.is_empty(),
            "Should return empty vec for empty delimited sections"
        );
    }

    #[test]
    fn test_extract_with_whitespace() {
        let input = "<first value> < second value >< third value>";
        let values = input.extract_delimited_values('<', '>');
        assert_eq!(
            values,
            ["first value", " second value ", " third value"],
            "Should extract three values including those with whitespace"
        );
    }

    #[test]
    fn test_common_prefix_basic() {
        let input = vec![
            "my-string-1".to_string(),
            "my-string-2".to_string(),
            "my-string-3".to_string(),
        ];
        assert_eq!(input.find_common_prefix(), Some("my-string".to_string()));
    }

    #[test]
    fn test_common_prefix_no_hyphen() {
        let input = vec![
            "prefix1".to_string(),
            "prefix2".to_string(),
            "prefix3".to_string(),
        ];
        assert_eq!(input.find_common_prefix(), None);
    }

    #[test]
    fn test_common_prefix_full_match() {
        let input = vec![
            "same-string".to_string(),
            "same-string".to_string(),
            "same-string".to_string(),
        ];
        assert_eq!(input.find_common_prefix(), Some("same-string".to_string()));
    }

    #[test]
    fn test_common_prefix_no_common() {
        let input = vec!["abc-def".to_string(), "xyz-ghi".to_string()];
        assert_eq!(input.find_common_prefix(), None);
    }

    #[test]
    fn test_common_prefix_empty_list() {
        let input = vec![];
        assert_eq!(input.find_common_prefix(), None);
    }

    #[test]
    fn test_common_prefix_single_element() {
        let input = vec!["only-one".to_string()];
        assert_eq!(input.find_common_prefix(), Some("only-one".to_string()));
    }
}
