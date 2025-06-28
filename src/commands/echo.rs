use std::io::Write;

#[inline(always)]
pub fn handle_echo(args: &Vec<String>, stdout_stream: &mut dyn Write) {
    let to_echo = args.join(" ");
    writeln!(stdout_stream, "{to_echo}").unwrap();
}
