use crate::commands::history::CommandHistory;
use crate::util::files::{find_completion_candidates_in_path, find_partial_completion};
use std::io::{stdin, stdout, PipeReader, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

// Note: can't use termion's stdout.cursor_pos() because it panics when running in Codecrafter's tests/containers.
// Manually setting cursor_pos was a huge pain/error-prone, so I opted for ANSI escape codes instead.
const RESET_CURSOR: &'static str = "\r\x1b[K";
const BELL_SOUND: &'static str = "\x07";

pub enum InputStream {
    Pipe(PipeReader),
    Stdin,
}

pub fn get_input_from_raw_mode(command_history: &CommandHistory) -> String {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{RESET_CURSOR}").unwrap();
    write!(stdout, "$ ").unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();
    let mut command_history_index = command_history.len();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\n') => {
                break;
            }

            Key::Up => {
                if !command_history.is_empty() && command_history_index > 0 {
                    command_history_index -= 1;
                    input =
                        String::from(command_history.get(command_history_index).unwrap().trim());
                }
            }

            Key::Down => {
                if !command_history.is_empty() && command_history_index < command_history.len() - 1
                {
                    command_history_index += 1;
                    input =
                        String::from(command_history.get(command_history_index).unwrap().trim());
                }
            }

            Key::Char('\t') => {
                if input.starts_with("ec") {
                    input = String::from("echo ");
                } else if input.starts_with("ex") {
                    input = String::from("exit ");
                } else {
                    let completions = find_completion_candidates_in_path(&input);

                    match completions.len() {
                        0 => write!(stdout, "{BELL_SOUND}").unwrap(),
                        1 => input = String::from(completions.first().unwrap().to_owned() + " "),
                        _ => {
                            write!(stdout, "{BELL_SOUND}").unwrap();
                            let partial_completion = find_partial_completion(&completions);

                            if partial_completion.is_empty() {
                                write!(stdout, "\n").unwrap();
                                write!(stdout, "{RESET_CURSOR}").unwrap();

                                for exe in completions {
                                    write!(stdout, "{exe}  ").unwrap();
                                }

                                write!(stdout, "\n").unwrap();
                            } else {
                                input = partial_completion;
                            }
                        }
                    }
                }
            }

            Key::Char(char) => {
                input.push(char);
            }
            _ => {}
        }

        reprint_current_line(&mut stdout, &input);
    }

    stdout.suspend_raw_mode().unwrap();
    write!(stdout, "\n").unwrap();
    input
}

#[inline(always)]
fn reprint_current_line(stdout: &mut RawTerminal<Stdout>, input: &String) {
    // Reset line
    write!(stdout, "{RESET_CURSOR}").unwrap();

    stdout.write("$ ".as_bytes()).unwrap();
    stdout.write(input.as_bytes()).unwrap();

    stdout.flush().unwrap()
}
