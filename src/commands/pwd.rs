use std::io::Write;
use crate::streams::stdin::RESET_CURSOR;

#[inline(always)]
pub fn handle_pwd(mut stdout_stream: Box<dyn Write>) {
    match std::env::current_dir() {
        Ok(path) => writeln!(stdout_stream, "{RESET_CURSOR}{}", path.display()).unwrap(),
        Err(err) => eprintln!("{err}"),
    }
}
