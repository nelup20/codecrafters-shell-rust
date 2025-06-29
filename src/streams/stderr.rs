use std::fs::OpenOptions;
use std::io::Write;

pub fn parse_stderr_redirect(args: &mut Vec<String>) -> Box<dyn Write> {
    match args.iter().position(|arg| arg == "2>" || arg == "2>>") {
        Some(index) => {
            let operator = args.get(index).unwrap();
            let should_append = operator == "2>>";
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
        None => Box::new(std::io::stderr()),
    }
}
