use crate::commands::command::Command;
use crate::streams::stdin::RESET_CURSOR;

#[inline(always)]
pub fn handle_exit(command: &mut Command) {
    let exit_code = command.args.pop().unwrap().parse().unwrap();
    print!("{RESET_CURSOR}");
    std::process::exit(exit_code);
}
