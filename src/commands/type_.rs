use crate::commands::command::Command;
use crate::commands::command_types::CommandType;
use crate::commands::command_types::CommandType::External;
use crate::util::files::find_in_path;
use std::io::Write;

#[inline(always)]
pub fn handle_type(mut command: Command) {
    let to_check = CommandType::from_str(command.args.pop().unwrap().as_str());
    match to_check {
        External(file) => match find_in_path(&file) {
            Some(file_path) => {
                writeln!(command.stdout_stream.as_writer(), "{file} is {file_path}").unwrap()
            }
            None => writeln!(command.stdout_stream.as_writer(), "{file}: not found").unwrap(),
        },
        _ => println!("{} is a shell builtin", to_check.as_str()),
    }
}
