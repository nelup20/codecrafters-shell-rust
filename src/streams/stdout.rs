use std::fs::File;

pub fn parse_stdout_redirect(args: &mut Vec<String>) -> Option<File> {
    let args_clone = args.clone();

    match args_clone.iter().position(|arg| arg == ">" || arg == "1>") {
        Some(index) => {
            args.remove(index);

            match args_clone.get(index + 1) {
                Some(file) => {
                    args.remove(index);
                    Some(File::create(file).unwrap())
                },
                None => panic!("Invalid redirect: output target missing")
            }
        },
        None => None
    }
}
