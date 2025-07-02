use crate::commands::command::Command;
use std::fs;
use std::io::BufRead;

#[inline(always)]
pub fn handle_history(command_history: &mut Vec<String>, command: Command) {
    match command.args.first() {
        None => {
            for (i, line) in command_history.iter().enumerate() {
                println!("{} {}", i + 1, line)
            }
        }

        Some(arg) => match arg.as_str() {
            "-r" => {
                let file_path = command.args.get(1).unwrap();
                for line in fs::read_to_string(file_path).unwrap().lines() {
                    command_history.push(String::from(line));
                }
            }

            "-w" => {
                let file_path = command.args.get(1).unwrap();
                // For the trailing newline
                command_history.push(String::from(""));
                fs::write(file_path, command_history.join("\n")).unwrap();
            }

            limit => {
                let limit: usize = limit.parse().unwrap();
                let history_length = command_history.len();

                for i in (history_length - limit)..history_length {
                    println!("{} {}", i + 1, command_history.get(i).unwrap());
                }
            }
        },
    }
}

pub fn add_to_history(history: &mut Vec<String>, command: &Command) {
    let mut line = String::new();
    line.push_str(command.command_type.as_str());
    line.push_str(" ");
    line.push_str(&command.args.join(" "));

    history.push(line);
}
