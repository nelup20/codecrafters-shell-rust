use crate::commands::command::Command;
use std::fs;

#[inline(always)]
pub fn handle_history(history: &mut Vec<String>, command: Command) {
    match command.args.first() {
        None => {
            for (i, line) in history.iter().enumerate() {
                println!("{} {}", i + 1, line)
            }
        }

        Some(arg) => match arg.as_str() {
            "-r" => {
                let file = command.args.get(1).unwrap();
                for line in fs::read_to_string(file).unwrap().lines() {
                    history.push(String::from(line));
                }
            }

            limit => {
                let limit: usize = limit.parse().unwrap();
                let history_length = history.len();

                for i in (history_length - limit)..history_length {
                    println!("{} {}", i + 1, history.get(i).unwrap());
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
