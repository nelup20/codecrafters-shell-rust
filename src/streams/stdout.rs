use std::fs::{File, OpenOptions};
use std::io::{stdout, PipeWriter, Write};

const REDIRECT_OPERATORS: [&str; 4] = [">", "1>", ">>", "1>>"];

pub enum OutputStream {
    File(File),
    Pipe(PipeWriter),
    Stdout
}

impl OutputStream {
    pub fn as_writer(&self) -> Box<dyn Write + '_> {
        match self {
            Self::File(file) => Box::new(file),
            Self::Pipe(pipe) => Box::new(pipe),
            Self::Stdout => Box::new(stdout())
        }
    }
}

pub fn parse_stdout_redirect(args: &mut Vec<String>) -> OutputStream {
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

            OutputStream::File(
                OpenOptions::new()
                    .write(true)
                    .append(should_append)
                    .create(true)
                    .open(file)
                    .unwrap(),
            )
        }
        None => OutputStream::Stdout,
    }
}
