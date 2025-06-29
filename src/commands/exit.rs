use crate::streams::stdin::RESET_CURSOR;

#[inline(always)]
pub fn handle_exit(args: &mut Vec<String>) {
    let exit_code = args.pop().unwrap().parse().unwrap();
    print!("{RESET_CURSOR}");
    std::process::exit(exit_code);
}
