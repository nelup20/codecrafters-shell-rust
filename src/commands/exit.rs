use crate::commands::command::Command;

#[inline(always)]
pub fn handle_exit(mut command: Command) {
    let exit_code = command.args.pop().unwrap().parse().unwrap();
    std::process::exit(exit_code);
}
