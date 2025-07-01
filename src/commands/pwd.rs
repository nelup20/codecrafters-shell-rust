use crate::commands::command::Command;
use std::io::Write;

#[inline(always)]
pub fn handle_pwd(command: Command) {
    match std::env::current_dir() {
        Ok(path) => writeln!(command.stdout_stream.as_writer(), "{}", path.display()).unwrap(),
        Err(err) => eprintln!("{err}"),
    }
}
