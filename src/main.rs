mod commands;
mod streams;
mod util;

use crate::commands::cd::handle_cd;
use crate::commands::echo::handle_echo;
use crate::commands::exit::handle_exit;
use crate::commands::other::handle_other;
use crate::commands::pwd::handle_pwd;
use crate::commands::type_::handle_type;
use crate::streams::stderr::parse_stderr_redirect;
use crate::streams::stdin::get_input_from_raw_mode;
use crate::streams::stdout::parse_stdout_redirect;
use crate::util::args::parse_args;
use commands::commands::Commands;
use termion::raw::IntoRawMode;

fn main() {
    loop {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        let input = get_input_from_raw_mode(stdin, &mut stdout);
        
        let delimiter = Commands::get_delimiter(&input);
        let (mut cmd, args) = input.split_once(delimiter).unwrap_or((&input, ""));
        if delimiter != " " {
            cmd = &cmd[1..];
        }

        let cmd = Commands::from_str(cmd.trim());
        let mut args = parse_args(args);

        let stdout_stream = parse_stdout_redirect(&mut args);
        let stderr_stream = parse_stderr_redirect(&mut args);
        
        match cmd {
            Commands::Pwd => handle_pwd(stdout_stream),
            Commands::Echo => handle_echo(&args, stdout_stream),
            Commands::Exit => handle_exit(&mut args),
            Commands::Type => handle_type(&mut args, stdout_stream),
            Commands::Cd => handle_cd(&mut args),
            Commands::Other(file) => {
                handle_other(&file, &args, stdout_stream, stderr_stream)
            }
        }
    }
}
