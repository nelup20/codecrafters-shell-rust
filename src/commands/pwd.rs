
#[inline(always)]
pub fn handle_pwd() {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("{err}"),
    }
}