use crate::commands::command::Command;
use std::io::Write;

#[inline(always)]
pub fn handle_cd(mut command: Command) {
    let mut path = command.args.pop().unwrap();

    if path.contains("~") {
        path = path.replace("~", std::env::home_dir().unwrap().to_str().unwrap());
    }

    match std::env::set_current_dir(&path) {
        Ok(_) => {}
        Err(_) => {
            writeln!(
                command.stderr_stream.as_writer(),
                "cd: {}: No such file or directory",
                &path
            )
            .unwrap();
        }
    };
}
