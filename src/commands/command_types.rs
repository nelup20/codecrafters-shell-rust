use crate::commands::builtin_commands::BuiltinCommands;
use crate::commands::command_types::CommandTypes::External;

pub enum CommandTypes {
    Builtin(BuiltinCommands),
    External(String),
}

impl CommandTypes {
    pub fn from_str(input: &str) -> Self {
        match input {
            "pwd" | "echo" | "exit" | "type" | "cd" => {
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
