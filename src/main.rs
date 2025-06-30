mod commands;
mod streams;
mod util;

use crate::commands::cd::handle_cd;
use crate::commands::command::Command;
use crate::commands::command_types::CommandTypes;
use crate::commands::echo::handle_echo;
use crate::commands::exit::handle_exit;
use crate::commands::other::handle_other;
use crate::commands::pwd::handle_pwd;
use crate::commands::type_::handle_type;
use crate::streams::stdin::get_input_from_raw_mode;
use commands::builtin_commands::BuiltinCommands;
use termion::raw::IntoRawMode;

fn main() {
    loop {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        let input = get_input_from_raw_mode(stdin, &mut stdout);
        let commands = Command::parse_commands(&input);
        
        for mut command in commands {
            match &command.command_type {
                CommandTypes::External(_) => handle_other(&mut command),
                CommandTypes::Builtin(cmd) => match cmd {
                    BuiltinCommands::Pwd => handle_pwd(&mut command),
                    BuiltinCommands::Echo => handle_echo(&mut command),
                    BuiltinCommands::Exit => handle_exit(&mut command),
                    BuiltinCommands::Type => handle_type(&mut command),
                    BuiltinCommands::Cd => handle_cd(&mut command),
                }
            }
        }
    }
}
