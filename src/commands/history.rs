use crate::commands::command::Command;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};
use std::ops::{Deref, DerefMut};

pub struct CommandHistory {
    pub last_appended_index: usize,
    history: Vec<String>,
}

impl CommandHistory {
    pub fn new() -> CommandHistory {
        let mut result = CommandHistory {
            history: Vec::new(),
            last_appended_index: 0,
        };

        match std::env::var("HISTFILE") {
            Err(_) => result,
            Ok(history_file_path) => {
                for line in fs::read_to_string(history_file_path).unwrap().lines() {
                    result.push(String::from(line));
                }

                result
            }
        }
    }
}

impl Deref for CommandHistory {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.history
    }
}

impl DerefMut for CommandHistory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.history
    }
}

#[inline(always)]
pub fn handle_history(command_history: &mut CommandHistory, command: Command) {
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

            "-a" => {
                let file_path = command.args.get(1).unwrap();
                let mut file = OpenOptions::new()
                    .write(false)
                    .append(true)
                    .create(true)
                    .open(file_path)
                    .unwrap();

                file.write_all(
                    command_history[command_history.last_appended_index..]
                        .join("\n")
                        .as_bytes(),
                )
                .unwrap();

                file.write_all("\n".as_bytes()).unwrap();

                command_history.last_appended_index = command_history.len();
            }

            "-w" => {
                let file_path = command.args.get(1).unwrap();
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(false)
                    .create(true)
                    .open(file_path)
                    .unwrap();

                file.write_all(command_history.join("\n").as_bytes())
                    .unwrap();
                file.write_all("\n".as_bytes()).unwrap();
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

pub fn add_to_history(command_history: &mut CommandHistory, command: &Command) {
    let mut line = String::new();
    line.push_str(command.command_type.as_str());
    line.push_str(" ");
    line.push_str(&command.args.join(" "));

    command_history.push(line);
}
