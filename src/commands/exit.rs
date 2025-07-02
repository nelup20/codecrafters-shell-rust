use crate::commands::command::Command;
use crate::commands::history::CommandHistory;

#[inline(always)]
pub fn handle_exit(command_history: &CommandHistory, mut command: Command) {
    command_history.write_to_history_file();
    let exit_code = command.args.pop().unwrap().parse().unwrap();
    std::process::exit(exit_code);
}
