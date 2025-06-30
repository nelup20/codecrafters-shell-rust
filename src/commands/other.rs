use crate::commands::command::Command;
use crate::streams::stdin::RESET_CURSOR;
use crate::util::files::find_in_path;
use std::io::Write;

#[inline(always)]
pub fn handle_other(command: &mut Command) {
    let file = command.command_type.as_str();
    match find_in_path(file) {
        Some(_) => {
            let output = std::process::Command::new(&file)
                .args(&command.args)
                .output()
                .expect("Child process didn't exit properly");

            if !&output.stdout.is_empty() {
                &command.stdout_stream.write(RESET_CURSOR.as_bytes());

                for char in output.stdout {
                    &command.stdout_stream.write(&[char]);
                    if char == '\n' as u8 {
                        &command.stdout_stream.write(RESET_CURSOR.as_bytes());
                    }
                }
            }

            if !&output.stderr.is_empty() {
                &command.stderr_stream.write(RESET_CURSOR.as_bytes());

                for char in output.stderr {
                    &command.stderr_stream.write(&[char]);
                    if char == '\n' as u8 {
                        &command.stderr_stream.write(RESET_CURSOR.as_bytes());
                    }
                }
            }
        }
        None => println!("{RESET_CURSOR}{file}: command not found"),
    }
}
