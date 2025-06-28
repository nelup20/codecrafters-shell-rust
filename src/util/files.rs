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

#[inline(always)]
fn file_has_execute_permission(file_path: &String) -> bool {
    fs::metadata(&file_path).unwrap().permissions().mode() & 0o111 != 0
}
