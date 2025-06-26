use crate::util::files::find_in_path;
use std::process::Command;

#[inline(always)]
pub fn handle_other(file: &str, args: &Vec<String>) {
    match find_in_path(&file) {
        Some(_) => {
            Command::new(&file)
                .args(args)
                .spawn()
                .expect(&format!("Failed to run {}", &file))
                .wait()
                .expect("Child process didn't exit properly");
        }
        None => println!("{file}: command not found"),
    }
}
