use crate::commands::command::Command;
use std::io::Write;

#[inline(always)]
pub fn handle_echo(command: Command) {
    let to_echo = command.args.join(" ");
    writeln!(command.stdout_stream.as_writer(), "{to_echo}").unwrap();
}
