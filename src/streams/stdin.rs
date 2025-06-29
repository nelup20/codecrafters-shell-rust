use crate::util::files::find_completion_candidates_in_path;
use std::io::{Stdin, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;

// Note: can't use termion's stdout.cursor_pos() because it panics when running in Codecrafter's tests/containers.
// Manually setting cursor_pos was a huge pain/error-prone, so I opted for ANSI escape codes instead.
pub const RESET_CURSOR: &'static str = "\r\x1b[K";
const BELL_SOUND: &'static str = "\x07";

pub fn get_input_from_raw_mode(stdin: Stdin, stdout: &mut RawTerminal<Stdout>) -> String {
    write!(stdout, "{RESET_CURSOR}").unwrap();
    write!(stdout, "$ ").unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();
    let mut tab_count = 0;

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\n') => {
                break;
            }

            Key::Char('\t') => {
                tab_count += 1;

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
                            if tab_count == 1 {
                                write!(stdout, "{BELL_SOUND}").unwrap();
                            } else {
                                write!(stdout, "\n").unwrap();
                                write!(stdout, "{RESET_CURSOR}").unwrap();

                                for exe in completions {
                                    write!(stdout, "{exe}  ").unwrap();
                                }

                                write!(stdout, "\n").unwrap();
                            }
                        }
                    }
                }
            }

            Key::Char(char) => {
                tab_count = 0;
                input.push(char);
            }
            _ => tab_count = 0,
        }

        reprint_current_line(stdout, &input);
    }

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
