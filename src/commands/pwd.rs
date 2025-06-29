use std::io::Write;

#[inline(always)]
pub fn handle_pwd(mut stdout_stream: Box<dyn Write>) {
    match std::env::current_dir() {
        Ok(path) => writeln!(stdout_stream, "{}", path.display()).unwrap(),
        Err(err) => eprintln!("{err}"),
    }
}
