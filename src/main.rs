use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

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
    match std::env::set_current_dir(path) {
        Ok(_) => {},
        Err(_) => eprintln!("cd: {path}: No such file or directory")
    };
}

#[inline(always)]
fn handle_pwd() {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("{err}")
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
            Command::new(file)
                .args(args.split(" "))
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
