use crate::commands::command_types::CommandType;
use crate::commands::input::parse_input;
use crate::streams::stderr::{parse_stderr_redirect, ErrorStream};
use crate::streams::stdin::InputStream;
use crate::streams::stdout::{parse_stdout_redirect, OutputStream};

pub struct Command {
    pub command_type: CommandType,
    pub args: Vec<String>,
    pub stdin_stream: InputStream,
    pub stdout_stream: OutputStream,
    pub stderr_stream: ErrorStream,
}

impl Command {
    pub fn parse_commands(input: &str) -> Vec<Command> {
        let mut result = Vec::new();

        if input.len() == 0 {
            return result;
        }

        let parsed_input = parse_input(input);

        let sub_commands: Vec<&[String]> = parsed_input
            .split(|arg| *arg == String::from("|"))
            .collect();
        
        for command_pairs in sub_commands.chunks(2) {
            if command_pairs.len() == 2 {
                let (pipe_reader, pipe_writer) = std::io::pipe().unwrap();

                let first_command = Command {
                    command_type: CommandType::from_str(&command_pairs[0][0]),
                    args: Vec::from(&command_pairs[0][1..]),
                    stdin_stream: InputStream::Stdin,
                    stdout_stream: OutputStream::Pipe(pipe_writer),
                    stderr_stream: ErrorStream::Stderr,
                };

                let second_command = Command {
                    command_type: CommandType::from_str(&command_pairs[1][0]),
                    args: Vec::from(&command_pairs[1][1..]),
                    stdin_stream: InputStream::Pipe(pipe_reader),
                    stdout_stream: OutputStream::Stdout,
                    stderr_stream: ErrorStream::Stderr,
                };

                result.push(first_command);
                result.push(second_command);
            } else {
                let mut args = Vec::from(&command_pairs[0][1..]);

                let stdin_stream = InputStream::Stdin;
                let stdout_stream = parse_stdout_redirect(&mut args);
                let stderr_stream = parse_stderr_redirect(&mut args);

                let command = Command {
                    command_type: CommandType::from_str(&command_pairs[0][0]),
                    args,
                    stdin_stream,
                    stdout_stream,
                    stderr_stream,
                };
                
                result.push(command);
            }
        }

        result
    }
}
