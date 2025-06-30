const DOUBLE_QUOTES_ESCAPE_CHARS: [char; 2] = ['\"', '\\'];
const SINGLE_QUOTES_ESCAPE_CHARS: [char; 1] = ['\''];
const OUTSIDE_QUOTES_ESCAPE_CHARS: [char; 4] = [' ', '\'', '\"', '\\'];

pub fn parse_input(args: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut is_inside_double_quotes = false;
    let mut is_inside_single_quotes = false;
    let mut prev_char_was_backslash = false;

    let mut current_arg = String::new();
    // note: chars aren't grapheme clusters, assuming input is ASCII
    for char in args.chars() {
        if char == '\n' {
            continue;
        }

        if char == '\\' {
            if prev_char_was_backslash || is_inside_single_quotes {
                current_arg.push('\\');
            }

            prev_char_was_backslash = !prev_char_was_backslash;
            continue;
        }

        if char == '"' && !prev_char_was_backslash && !is_inside_single_quotes {
            is_inside_double_quotes = !is_inside_double_quotes;
        }

        if char == '\'' && !prev_char_was_backslash && !is_inside_double_quotes {
            is_inside_single_quotes = !is_inside_single_quotes;
        }

        if prev_char_was_backslash {
            if is_inside_double_quotes && !DOUBLE_QUOTES_ESCAPE_CHARS.contains(&char) {
                current_arg.push('\\');
                current_arg.push(char);
            } else {
                current_arg.push(char);
            }

            prev_char_was_backslash = false;
        } else {
            if is_arg_separating_space(
                is_inside_double_quotes,
                is_inside_single_quotes,
                &current_arg,
                char,
            ) {
                result.push(current_arg);
                current_arg = String::new();
                continue;
            }

            if char_doesnt_require_escaping(is_inside_double_quotes, is_inside_single_quotes, &char)
            {
                current_arg.push(char);
            }
        }
    }

    if !current_arg.trim().is_empty() {
        result.push(current_arg)
    }

    result
}

#[inline(always)]
fn char_doesnt_require_escaping(
    is_inside_double_quotes: bool,
    is_inside_single_quotes: bool,
    char: &char,
) -> bool {
    (is_inside_double_quotes && !DOUBLE_QUOTES_ESCAPE_CHARS.contains(&char))
        || (is_inside_single_quotes && !SINGLE_QUOTES_ESCAPE_CHARS.contains(&char))
        || !OUTSIDE_QUOTES_ESCAPE_CHARS.contains(&char)
}

#[inline(always)]
fn is_arg_separating_space(
    is_inside_double_quotes: bool,
    is_inside_single_quotes: bool,
    current_arg: &String,
    char: char,
) -> bool {
    char == ' '
        && !is_inside_double_quotes
        && !is_inside_single_quotes
        && !current_arg.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_unescaped_single_quotes() {
        assert_eq!(vec!["some item"], parse_input("\'some item\'"));
    }

    #[test]
    fn parses_unescaped_double_quotes() {
        assert_eq!(vec!["some item"], parse_input("\"some item\""));
    }

    #[test]
    fn parses_escaped_single_quotes() {
        assert_eq!(vec!["\'some", "item\'"], parse_input("\\\'some item\\\'"));
    }

    #[test]
    fn parses_escaped_double_quotes() {
        assert_eq!(vec!["\"some", "item\""], parse_input("\\\"some item\\\""));
    }

    #[test]
    fn parses_literal_backslashes_within_single_quotes_1() {
        assert_eq!(vec!["some\\\\\\item"], parse_input("\'some\\\\\\item\'"));
    }

    #[test]
    fn parses_literal_backslashes_within_single_quotes_2() {
        assert_eq!(
            vec!["some\\\"test\\\"item"],
            parse_input("\'some\\\"test\\\"item\'")
        );
    }

    #[test]
    fn parses_escaped_backslashes_within_double_quotes() {
        assert_eq!(vec!["some\\item"], parse_input("\"some\\\\item\""));
    }

    #[test]
    fn parses_escaped_double_quotes_within_double_quotes() {
        assert_eq!(vec!["some\"item"], parse_input("\"some\\\"item\""));
    }

    #[test]
    fn parses_backslash_with_non_escaped_chars_within_double_quotes() {
        assert_eq!(vec!["\\5\\o"], parse_input("\"\\5\\o\""));
    }

    #[test]
    fn parses_backslash_with_non_escaped_chars_within_single_quotes() {
        assert_eq!(vec!["\\5\\o"], parse_input("\'\\5\\o\'"));
    }

    #[test]
    fn parses_backslash_with_non_escaped_chars_outside_of_quotes() {
        assert_eq!(vec!["5o"], parse_input("\\5\\o"));
    }

    #[test]
    fn parses_backslash_with_single_quote_within_double_quotes() {
        assert_eq!(vec!["\\23\\'"], parse_input("\"\\23\\'\""));
    }

    #[test]
    fn parses_escaped_and_unescaped_double_quotes() {
        assert_eq!(
            vec!["some item", "\"another", "item\""],
            parse_input("\"some item\" \\\"another item\\\"")
        );
    }
}
