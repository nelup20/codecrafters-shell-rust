
#[inline(always)]
pub fn handle_exit(args: &mut Vec<String>) {
    let exit_code = args.pop().unwrap().parse().unwrap();
    std::process::exit(exit_code);
}
