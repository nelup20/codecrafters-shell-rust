use crate::commands::commands::Commands;
use crate::util::files::find_in_path;
use std::fs::File;
use std::io::{stdout, Write};

// TODO: refactor to use dyn Write (though can't with Stdio) or something else
#[inline(always)]
pub fn handle_type(args: &mut Vec<String>, stdout_file: Option<File>) {
    let to_check = Commands::from_str(args.pop().unwrap().as_str());
    match to_check {
        Commands::Other(file) => match find_in_path(&file) {
            Some(file_path) => match stdout_file {
                None => {
                    writeln!(stdout(), "{file} is {file_path}").unwrap();
                }
                Some(mut redirect_file) => {
                    writeln!(redirect_file, "{file} is {file_path}").unwrap()
                }
            },

            None => match stdout_file {
                None => writeln!(stdout(), "{file}: not found").unwrap(),
                Some(mut redirect_file) => {
                    writeln!(redirect_file, "{file}: not found").unwrap();
                }
            },
        },
        _ => println!("{} is a shell builtin", to_check.as_str()),
    }
}
