mod commands;
mod streams;
mod util;

use crate::commands::cd::handle_cd;
use crate::commands::command::Command;
use crate::commands::command_types::CommandType;
use crate::commands::echo::handle_echo;
use crate::commands::exit::handle_exit;
use crate::commands::external::handle_external;
use crate::commands::pwd::handle_pwd;
use crate::commands::type_::handle_type;
use crate::streams::stdin::get_input_from_raw_mode;
use commands::builtin_commands::BuiltinCommands;
use crate::commands::history::{add_to_history, handle_history};

fn main() {
    let mut command_history: Vec<String> = Vec::new();
    
    loop {
        let input = get_input_from_raw_mode(&command_history);
        let commands = Command::parse_commands(&input);
        
        for command in commands {
            add_to_history(&mut command_history, &command);
            
            match &command.command_type {
                CommandType::External(_) => handle_external(command),
                CommandType::Builtin(cmd) => match cmd {
                    BuiltinCommands::Pwd => handle_pwd(command),
                    BuiltinCommands::Echo => handle_echo(command),
                    BuiltinCommands::Exit => handle_exit(command),
                    BuiltinCommands::Type => handle_type(command),
                    BuiltinCommands::Cd => handle_cd(command),
                    BuiltinCommands::History => handle_history(&command_history, command),
                }
            }
        }
    }
}
