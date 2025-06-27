use crate::util::files::find_in_path;
use std::fs::File;
use std::process::{Command, Stdio};

#[inline(always)]
pub fn handle_other(file: &str, args: &Vec<String>, stdout_file: Option<File>, stderr_file: Option<File>) {
    let stdout = if stdout_file.is_some() {
        Stdio::from(stdout_file.unwrap())
    } else {
        Stdio::from(std::io::stdout())
    };

    let stderr = if stderr_file.is_some() {
        Stdio::from(stderr_file.unwrap())
    } else {
        Stdio::from(std::io::stderr())
    };
    
    match find_in_path(&file) {
        Some(_) => {
            Command::new(&file)
                .stdout(stdout)
                .stderr(stderr)
                .args(args)
                .spawn()
                .expect(&format!("Failed to run {}", &file))
                .wait()
                .expect("Child process didn't exit properly");
        }
        None => println!("{file}: command not found"),
    }
}
