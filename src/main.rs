use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().replace("''", "");
        let input = input.as_str();

        match input {
            "pwd" => handle_pwd(),
            cmd if cmd.starts_with("echo") => handle_echo(cmd),
            cmd if cmd.starts_with("exit") => handle_exit(cmd),
            cmd if cmd.starts_with("type") => handle_type(cmd),
            cmd if cmd.starts_with("cd") => handle_cd(cmd),
            cmd => handle_unknown(cmd),
        }
    }
}

#[inline(always)]
fn handle_cd(cmd: &str) {
    let (_, path) = cmd.split_once(" ").unwrap();
    let mut path = String::from(path);

    if path.contains("~") {
        path = path.replace("~", std::env::home_dir().unwrap().to_str().unwrap());
    }

    match std::env::set_current_dir(&path) {
        Ok(_) => {}
        Err(_) => eprintln!("cd: {}: No such file or directory", &path),
    };
}

#[inline(always)]
fn handle_pwd() {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("{err}"),
    }
}

#[inline(always)]
fn handle_exit(cmd: &str) {
    let exit_code = cmd.split_once(" ").unwrap().1.parse().unwrap();
    std::process::exit(exit_code);
}

#[inline(always)]
fn handle_echo(cmd: &str) {
    let (_, to_echo) = cmd.split_once(" ").unwrap();
    
    let to_echo = if to_echo.contains("'") {
        to_echo.replace("'", "")
    } else {
        parse_non_quoted_args(&to_echo).join("")
    };

    println!("{to_echo}");
}

#[inline(always)]
fn handle_type(cmd: &str) {
    let (_, to_check) = cmd.split_once(" ").unwrap();
    match to_check {
        "echo" | "exit" | "type" | "pwd" | "cd" => println!("{to_check} is a shell builtin"),
        _ => match find_in_path(to_check) {
            Some(file_path) => println!("{to_check} is {file_path}"),
            None => println!("{to_check}: not found"),
        },
    }
}

#[inline(always)]
fn handle_unknown(cmd: &str) {
    let (file, args) = cmd.split_once(" ").unwrap_or((cmd, ""));
    match find_in_path(file) {
        Some(_) => {
            let args: Vec<String> = if args.contains("'") {
                args.split("' ").map(|arg| arg.replace("'", "")).collect()
            } else {
                parse_non_quoted_args(args)
            };

            Command::new(file)
                .args(args)
                .spawn()
                .expect(&format!("Failed to run {cmd}"))
                .wait()
                .expect("Child process didn't exit properly");
        }
        None => println!("{file}: command not found"),
    }
}

fn find_in_path(file: &str) -> Option<String> {
    match std::env::var("PATH") {
        Ok(paths) => {
            for path in paths.split(":") {
                let file_path = format!("{path}/{file}");
                if fs::exists(&file_path).unwrap() {
                    return Some(file_path);
                }
            }
        }
        Err(_) => println!("PATH environment variable is not set."),
    }

    None
}

fn parse_non_quoted_args(args: &str) -> Vec<String> {
    args.split(" ")
        .filter(|arg| !arg.trim().is_empty())
        .map(|arg| arg.to_owned() + " ")
        .collect()
}
