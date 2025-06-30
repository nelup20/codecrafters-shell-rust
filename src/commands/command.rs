use crate::commands::command_types::CommandType;
use crate::commands::input::parse_input;
use crate::streams::stderr::parse_stderr_redirect;
use crate::streams::stdout::parse_stdout_redirect;
use std::io::Write;

pub struct Command {
    pub command_type: CommandType,
    pub args: Vec<String>,
    pub stdout_stream: Box<dyn Write>,
    pub stderr_stream: Box<dyn Write>,
}

impl Command {
    pub fn parse_commands(input: &str) -> Vec<Command> {
        let mut result = Vec::new();

        // TODO: piping stage not yet actually implemented (stdin/stdout redirection)
        let parsed_input = parse_input(input);
        for sub_command in parsed_input.split(|arg| *arg == String::from("|")) {
            let mut args = Vec::from(&sub_command[1..]);

            let stdout_stream = parse_stdout_redirect(&mut args);
            let stderr_stream = parse_stderr_redirect(&mut args);

            let command = Command {
                command_type: CommandType::from_str(&sub_command[0]),
                stdout_stream,
                stderr_stream,
                args,
            };

            result.push(command)
        }

        result
    }
}
