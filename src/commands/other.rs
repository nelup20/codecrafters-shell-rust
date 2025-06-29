use crate::util::files::find_in_path;
use std::io::Write;
use std::process::Command;
use crate::streams::stdin::RESET_CURSOR;

#[inline(always)]
pub fn handle_other(
    file: &str,
    args: &Vec<String>,
    mut stdout_stream: Box<dyn Write>,
    mut stderr_stream: Box<dyn Write>,
) {
    match find_in_path(&file) {
        Some(_) => {
            let output = Command::new(&file)
                .args(args)
                .output()
                .expect("Child process didn't exit properly");

            if !&output.stdout.is_empty() {
                for char in output.stdout {
                    stdout_stream.write(&[char]);
                    if char == '\n' as u8 {
                        stdout_stream.write(RESET_CURSOR.as_bytes());
                    }
                }
            }

            if !&output.stderr.is_empty() {
                stderr_stream.write(RESET_CURSOR.as_bytes());
                
                for char in output.stderr {
                    stderr_stream.write(&[char]);
                    if char == '\n' as u8 {
                        stderr_stream.write(RESET_CURSOR.as_bytes());
                    }
                }
            }
        }
        None => println!("{file}: command not found"),
    }
}
