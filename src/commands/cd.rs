
#[inline(always)]
pub fn handle_cd(args: &mut Vec<String>) {
    let mut path = args.pop().unwrap();

    if path.contains("~") {
        path = path.replace("~", std::env::home_dir().unwrap().to_str().unwrap());
    }

    match std::env::set_current_dir(&path) {
        Ok(_) => {}
        Err(_) => eprintln!("cd: {}: No such file or directory", &path),
    };
}
