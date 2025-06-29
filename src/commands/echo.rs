use crate::streams::stdin::RESET_CURSOR;
use std::io::Write;

#[inline(always)]
pub fn handle_echo(args: &Vec<String>, mut stdout_stream: Box<dyn Write>) {
    let to_echo = args.join(" ");
    writeln!(stdout_stream, "{RESET_CURSOR}{to_echo}").unwrap();
}
