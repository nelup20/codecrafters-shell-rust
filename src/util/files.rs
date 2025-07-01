use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn find_in_path(file: &str) -> Option<String> {
    match std::env::var("PATH") {
        Ok(paths) => {
            for path in paths.split(":") {
                let file_path = format!("{path}/{file}");
                if fs::exists(&file_path).unwrap() && file_has_execute_permission(&file_path) {
                    return Some(file_path);
                }
            }
        }
        Err(_) => println!("PATH environment variable is not set."),
    }

    None
}

// TODO: save binaries found in PATH at startup instead of checking every time
// Also maybe implement Trie since existing crates weren't that great/ergonomic
pub fn find_completion_candidates_in_path(file: &str) -> Vec<String> {
    let mut result = Vec::new();

    match std::env::var("PATH") {
        Ok(paths) => {
            for path in paths.split(":") {
                match fs::read_dir(path) {
                    Ok(dir) => {
                        for dir_entry in dir {
                            match dir_entry {
                                Ok(entry) => {
                                    let file_name =
                                        String::from(entry.file_name().to_str().unwrap());

                                    if file_name.starts_with(file)
                                        && entry.metadata().unwrap().is_file()
                                        && file_has_execute_permission(
                                            entry.path().to_str().unwrap(),
                                        )
                                        && !result.contains(&file_name)
                                    {
                                        result.push(file_name);
                                    }
                                }
                                Err(err) => eprintln!("Error with dir_entry: {err}"),
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => println!("PATH environment variable is not set."),
    }

    result.sort();
    result
}

pub fn find_partial_completion(input: &Vec<String>) -> String {
    let mut result = String::new();

    if !input.is_empty() {
        let shortest_cmd = input.first().unwrap();
        let mut all_contain_shortest = true;

        for command in input {
            if !command.contains(shortest_cmd) {
                all_contain_shortest = false;
                break;
            }
        }

        if all_contain_shortest {
            result = shortest_cmd.clone();
        }
    }

    result
}

#[inline(always)]
fn file_has_execute_permission(file_path: &str) -> bool {
    fs::metadata(&file_path).unwrap().permissions().mode() & 0o111 != 0
}
