
pub fn parse_args(args: &str) -> Vec<String> {
    let args = args.trim().replace("''", "").replace("\"\"", "");

    let separator = if args.contains("\""){
        "\""
    } else if args.contains("'") {
        "'"
    } else {
        " "
    };

    args.split(separator)
        .filter(|arg| !arg.trim().is_empty())
        .map(|arg| arg.trim().to_owned())
        .collect()
}
