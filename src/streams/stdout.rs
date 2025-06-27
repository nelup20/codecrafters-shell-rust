use std::fs::{File, OpenOptions};

pub fn parse_stdout_redirect(args: &mut Vec<String>) -> Option<File> {
    let args_clone = args.clone();

    match args_clone
        .iter()
        .position(|arg| arg == ">" || arg == "1>" || arg == ">>" || arg == "1>>")
    {
        Some(index) => {
            let operator = args_clone.get(index).unwrap();
            let should_append = operator == ">>" || operator == "1>>";
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
                None => panic!("Invalid stdout redirect: output target missing"),
            }
        }
        None => None,
    }
}
