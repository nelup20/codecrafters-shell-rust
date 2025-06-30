use crate::commands::command::Command;
use crate::commands::command_types::CommandType;
use crate::commands::command_types::CommandType::External;
use crate::streams::stdin::RESET_CURSOR;
use crate::util::files::find_in_path;
use std::io::Write;

#[inline(always)]
pub fn handle_type(command: &mut Command) {
    let to_check = CommandType::from_str(command.args.pop().unwrap().as_str());
    match to_check {
        External(file) => match find_in_path(&file) {
            Some(file_path) => {
                writeln!(command.stdout_stream, "{RESET_CURSOR}{file} is {file_path}").unwrap()
            }
            None => writeln!(command.stdout_stream, "{RESET_CURSOR}{file}: not found").unwrap(),
        },
        _ => println!("{RESET_CURSOR}{} is a shell builtin", to_check.as_str()),
    }
}
