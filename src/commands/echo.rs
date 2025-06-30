use crate::commands::command::Command;
use crate::streams::stdin::RESET_CURSOR;
use std::io::Write;

#[inline(always)]
pub fn handle_echo(command: &mut Command) {
    let to_echo = command.args.join(" ");
    writeln!(command.stdout_stream, "{RESET_CURSOR}{to_echo}").unwrap();
}
