use std::fs::File;
use std::io::{stdout, Write};

#[inline(always)]
pub fn handle_echo(args: &Vec<String>, stdout_file: Option<File>) {
    let to_echo = args.join(" ");

    match stdout_file {
        None => {
            writeln!(stdout(), "{to_echo}").unwrap();
        }
        
        Some(mut file) => {
            writeln!(file, "{to_echo}").unwrap();
        }
    }
}
