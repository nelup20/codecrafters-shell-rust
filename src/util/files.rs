use std::fs;
use std::os::unix::fs::PermissionsExt;
use crate::streams::stdin::RESET_CURSOR;

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

pub fn find_completion_candidate_in_path(file: &str) -> Option<String> {
    match std::env::var("PATH") {
        Ok(paths) => {
            for path in paths.split(":") {
                match fs::read_dir(path) {
                    Ok(dir) => {
                        for dir_entry in dir {
                            match dir_entry {
                                Ok(entry) => {
                                    if entry.file_name().to_str()?.starts_with(file)
                                        && entry.metadata().unwrap().is_file()
                                        && file_has_execute_permission(entry.path().to_str()?)
                                    {
                                        return Some(String::from(entry.file_name().to_str()?));
                                    }
                                }
                                Err(err) => eprintln!("{RESET_CURSOR}Error with dir_entry: {err}")
                            }
                        }
                    }
                    Err(err) => eprintln!("{RESET_CURSOR}Error reading dir with path: {path}. Error: {err}")
                }
            }
        }
        Err(_) => println!("PATH environment variable is not set."),
    }

    None
}

#[inline(always)]
fn file_has_execute_permission(file_path: &str) -> bool {
    fs::metadata(&file_path).unwrap().permissions().mode() & 0o111 != 0
}
