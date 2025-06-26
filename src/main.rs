mod commands;
mod util;

use crate::commands::cd::handle_cd;
use crate::commands::echo::handle_echo;
use crate::commands::exit::handle_exit;
use crate::commands::other::handle_other;
use crate::commands::pwd::handle_pwd;
use crate::commands::type_::handle_type;
use crate::util::args::parse_args;
use commands::commands::Commands;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let (cmd, args) = input.split_once(" ").unwrap_or((&input, ""));
        let cmd = Commands::from_str(cmd.trim());
        let mut args = parse_args(args);

        match cmd {
            Commands::Pwd => handle_pwd(),
            Commands::Echo => handle_echo(&args),
            Commands::Exit => handle_exit(&mut args),
            Commands::Type => handle_type(&mut args),
            Commands::Cd => handle_cd(&mut args),
            Commands::Other(file) => handle_other(&file, &args),
        }
    }
}

