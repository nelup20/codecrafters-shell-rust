use std::fs::OpenOptions;
use std::io::Write;

const REDIRECT_OPERATORS: [&str; 4] = [">", "1>", ">>", "1>>"];

pub fn parse_stdout_redirect(args: &mut Vec<String>) -> Box<dyn Write> {
    match args
        .iter()
        .position(|arg| REDIRECT_OPERATORS.contains(&&**arg))
    {
        Some(index) => {
            let operator = args.get(index).unwrap();
            let should_append = operator.contains(">>");
            args.remove(index);

            let file = args.get(index).unwrap().clone();
            args.remove(index);

            Box::new(
                OpenOptions::new()
                    .write(true)
                    .append(should_append)
                    .create(true)
                    .open(file)
                    .unwrap(),
            )
        }
        None => Box::new(std::io::stdout()),
    }
}
