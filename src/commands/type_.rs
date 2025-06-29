use crate::commands::commands::Commands;
use crate::util::files::find_in_path;
use std::io::Write;
use crate::streams::stdin::RESET_CURSOR;

#[inline(always)]
pub fn handle_type(args: &mut Vec<String>, mut stdout_stream: Box<dyn Write>) {
    let to_check = Commands::from_str(args.pop().unwrap().as_str());
    match to_check {
        Commands::Other(file) => match find_in_path(&file) {
            Some(file_path) => writeln!(stdout_stream, "{RESET_CURSOR}{file} is {file_path}").unwrap(),
            None => writeln!(stdout_stream, "{RESET_CURSOR}{file}: not found").unwrap(),
        },
        _ => println!("{RESET_CURSOR}{} is a shell builtin", to_check.as_str()),
    }
}
