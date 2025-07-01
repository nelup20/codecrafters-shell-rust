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

        let (mut pipe_reader, _) = std::io::pipe().unwrap();

        for (i, sub_cmd) in sub_commands.iter().enumerate() {
            let command_type = CommandType::from_str(&sub_cmd[0]);
            let mut args = Vec::from(&sub_cmd[1..]);
            let mut stdin_stream = InputStream::Stdin;
            let mut stdout_stream = OutputStream::Stdout;
            let mut stderr_stream = ErrorStream::Stderr;

            if i == 0 && sub_commands.len() > 1 {
                let (reader_for_next_cmd, writer) = std::io::pipe().unwrap();

                stdout_stream = OutputStream::Pipe(writer);
                pipe_reader = reader_for_next_cmd;
            }

            if i != 0 && i != sub_commands.len() - 1 {
                let (reader_for_next_cmd, writer) = std::io::pipe().unwrap();

                stdin_stream = InputStream::Pipe(pipe_reader);
                stdout_stream = OutputStream::Pipe(writer);

                pipe_reader = reader_for_next_cmd;
            }

            if i == sub_commands.len() - 1 {
                stdout_stream = parse_stdout_redirect(&mut args);
                stderr_stream = parse_stderr_redirect(&mut args);

                if sub_commands.len() > 1 {
                    stdin_stream = InputStream::Pipe(pipe_reader.try_clone().unwrap());
                }
            }

            result.push(Command {
                command_type,
                args,
                stdin_stream,
                stdout_stream,
                stderr_stream,
            })
        }

        result
    }
}
