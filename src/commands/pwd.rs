use std::fs::File;
use std::io::{stdout, Write};

#[inline(always)]
pub fn handle_pwd(stdout_file: Option<File>) {
    match std::env::current_dir() {
        Ok(path) => match stdout_file {
            None => {
                writeln!(stdout(), "{}", path.display()).unwrap();
            }

            Some(mut file) => {
                writeln!(file, "{}", path.display()).unwrap();
            }
        },
        Err(err) => eprintln!("{err}"),
    }
}
