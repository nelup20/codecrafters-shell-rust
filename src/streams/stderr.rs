use crate::util::files::open_file_to_write;
use std::fs::File;
use std::io::{stderr, Write};

pub enum ErrorStream {
    File(File),
    Stderr,
}

impl ErrorStream {
    pub fn as_writer(&self) -> Box<dyn Write + '_> {
        match self {
            ErrorStream::File(file) => Box::new(file),
            ErrorStream::Stderr => Box::new(stderr()),
        }
    }
}

pub fn parse_stderr_redirect(args: &mut Vec<String>) -> ErrorStream {
    match args.iter().position(|arg| arg == "2>" || arg == "2>>") {
        Some(index) => {
            let operator = args.get(index).unwrap();
            let should_append = operator == "2>>";
            args.remove(index);

            let file = args.get(index).unwrap().clone();
            args.remove(index);

            ErrorStream::File(open_file_to_write(&file, should_append))
        }
        None => ErrorStream::Stderr,
    }
}
