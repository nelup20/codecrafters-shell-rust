use crate::util::files::find_in_path;
use std::process::Stdio;
use crate::commands::command::Command;
use crate::streams::stderr::ErrorStream;
use crate::streams::stdin::InputStream;
use crate::streams::stdout::OutputStream;

#[inline(always)]
pub fn handle_external(command: Command) {
    let executable = command.command_type.as_str();
    
    match find_in_path(executable) {
        Some(_) => {
            let mut child_process = std::process::Command::new(executable);
            child_process.args(command.args);

            match command.stdin_stream {
                InputStream::Pipe(pipe) => {
                    child_process.stdin(Stdio::from(pipe));
                }

                InputStream::Stdin => {}
            }

            match command.stderr_stream {
                ErrorStream::File(file) => {
                    child_process.stderr(Stdio::from(file));
                }
                ErrorStream::Stderr => {}
            }

            match command.stdout_stream {
                OutputStream::File(file) => {
                    child_process.stdout(Stdio::from(file));
                    child_process.spawn().unwrap().wait().unwrap();
                }

                OutputStream::Pipe(pipe) => {
                    child_process.stdout(Stdio::from(pipe));
                    std::thread::spawn(move || {
                        child_process.spawn().unwrap().wait().unwrap();
                    });
                }

                OutputStream::Stdout => {
                    child_process.spawn().unwrap().wait().unwrap();
                }
            }
        }
        None => println!("{executable}: command not found"),
    }
}
