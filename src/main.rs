mod commands;
mod streams;
mod util;

use crate::commands::cd::handle_cd;
use crate::commands::echo::handle_echo;
use crate::commands::exit::handle_exit;
use crate::commands::other::handle_other;
use crate::commands::pwd::handle_pwd;
use crate::commands::type_::handle_type;
use crate::streams::stdout::parse_stdout_redirect;
use crate::util::args::parse_args;
use commands::commands::Commands;
use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let delimiter = Commands::get_delimiter(&input);
        let (mut cmd, args) = input.split_once(delimiter).unwrap_or((&input, ""));
        if delimiter != " " {
            cmd = &cmd[1..];
        }
        
        let cmd = Commands::from_str(cmd.trim());
        let mut args = parse_args(args);
        let redirect_file = parse_stdout_redirect(&mut args);

        match cmd {
            Commands::Pwd => handle_pwd(redirect_file),
            Commands::Echo => handle_echo(&args, redirect_file),
            Commands::Exit => handle_exit(&mut args),
            Commands::Type => handle_type(&mut args, redirect_file),
            Commands::Cd => handle_cd(&mut args),
            Commands::Other(file) => handle_other(&file, &args, redirect_file),
        }
    }
}
