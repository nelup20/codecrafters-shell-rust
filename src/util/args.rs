pub fn parse_args(args: &str) -> Vec<String> {
    let args = args.trim().replace("''", "").replace("\"\"", "");

    // TODO: doesn't actually handle escaping properly (despite passing the stage)
    // ex:
    // input    --> echo "some item" \"another item\"
    // expected --> some item "another item"
    // actual   --> "some item" "another item"
    if contains_unescaped_double_quotes(&args) {
        split_by_separator(args, "\"")
    } else if contains_unescaped_single_quotes(&args) {
        split_by_separator(args, "'")
    } else if !args.contains("\\") {
        split_by_separator(args, " ")
    } else {
        return args
            .replace("\\", "")
            .split(" ")
            .map(|arg| arg.to_owned())
            .collect();
    }
}

#[inline(always)]
fn contains_unescaped_single_quotes(args: &str) -> bool {
    args.contains("'") && !args.contains("\\\'")
}

#[inline(always)]
fn contains_unescaped_double_quotes(args: &str) -> bool {
    args.contains("\"") && !args.contains("\\\"")
}

fn split_by_separator(args: String, separator: &str) -> Vec<String> {
    args.split(separator)
        .filter(|arg| !arg.trim().is_empty())
        .map(|arg| arg.trim().to_owned())
        .collect()
}
