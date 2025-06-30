use crate::commands::command::Command;
use crate::streams::stdin::RESET_CURSOR;
use std::io::Write;

#[inline(always)]
pub fn handle_pwd(command: &mut Command) {
    match std::env::current_dir() {
        Ok(path) => writeln!(command.stdout_stream, "{RESET_CURSOR}{}", path.display()).unwrap(),
        Err(err) => eprintln!("{err}"),
    }
}
