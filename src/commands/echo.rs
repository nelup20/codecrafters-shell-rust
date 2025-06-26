
#[inline(always)]
pub fn handle_echo(args: &Vec<String>) {
    let to_echo = args.join(" ");

    println!("{to_echo}");
}
