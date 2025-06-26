#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            cmd if cmd.starts_with("echo") => handle_echo(cmd),
            cmd if cmd.starts_with("exit") => handle_exit(cmd),
            cmd if cmd.starts_with("type") => handle_type(cmd),
            cmd => handle_unknown(cmd)
        }
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
        "echo" | "exit" | "type" => println!("{to_check} is a shell builtin"),
        _ => println!("{to_check}: not found")
    }
}

#[inline(always)]
fn handle_unknown(cmd: &str) {
    println!("{cmd}: command not found");
}