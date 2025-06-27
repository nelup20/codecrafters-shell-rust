use std::fs::{File, OpenOptions};

pub fn parse_stderr_redirect(args: &mut Vec<String>) -> Option<File> {
    let args_clone = args.clone();

    match args_clone
        .iter()
        .position(|arg| arg == "2>" || arg == "2>>")
    {
        Some(index) => {
            let operator = args_clone.get(index).unwrap();
            let should_append = operator == "2>>";
            args.remove(index);

            match args_clone.get(index + 1) {
                Some(file) => {
                    args.remove(index);

                    Some(
                        OpenOptions::new()
                            .write(true)
                            .append(should_append)
                            .create(true)
                            .open(file)
                            .unwrap(),
                    )
                }
                None => panic!("Invalid stderr redirect: output target missing"),
            }
        }
        None => None,
    }
}
