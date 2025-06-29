use crate::streams::stdin::RESET_CURSOR;
use std::io::Write;

#[inline(always)]
pub fn handle_cd(args: &mut Vec<String>, mut stderr_stream: Box<dyn Write>) {
    let mut path = args.pop().unwrap();

    if path.contains("~") {
        path = path.replace("~", std::env::home_dir().unwrap().to_str().unwrap());
    }

    match std::env::set_current_dir(&path) {
        Ok(_) => {}
        Err(_) => {
            writeln!(
                stderr_stream,
                "{RESET_CURSOR}cd: {}: No such file or directory",
                &path
            )
            .unwrap();
        }
    };
}
