use crate::commands::builtin_commands::BuiltinCommands;
use crate::commands::command_types::CommandType::External;

pub enum CommandType {
    Builtin(BuiltinCommands),
    External(String),
}

impl CommandType {
    pub fn from_str(input: &str) -> Self {
        match input {
            "pwd" | "echo" | "exit" | "type" | "cd" | "history" => {
                Self::Builtin(BuiltinCommands::from_str(input))
            }
            command => External(String::from(command)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Builtin(command) => command.as_str(),
            Self::External(command) => command,
        }
    }
}
