#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.starts_with("exit") {
            let exit_code = input.split_once(" ").unwrap().1.parse().unwrap();
            std::process::exit(exit_code);
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}
