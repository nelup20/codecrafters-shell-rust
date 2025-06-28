use std::io::{Stdin, Stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;

// Note: can't use termion's stdout.cursor_pos() because it panics when running in Codecrafter's tests/containers.
// Manually setting cursor_pos was a huge pain/error-prone, so I opted for ANSI escape codes instead.
pub const RESET_CURSOR: &'static str = "\r\x1b[K";

pub fn get_input_from_raw_mode(stdin: Stdin, stdout: &mut RawTerminal<Stdout>) -> String {
    write!(stdout, "{RESET_CURSOR}").unwrap();
    write!(stdout, "$ ").unwrap();
    stdout.flush().unwrap();

    let mut input = String::new();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\n') => {
                break;
            }

            Key::Char('\t') => {
                if input.starts_with("ec") {
                    input = String::from("echo ");
                }

                if input.starts_with("ex") {
                    input = String::from("exit ");
                }
            }

            Key::Char(char) => {
                input.push(char);
            }
            _ => {}
        }

        reprint_current_line(stdout, &input);
    }

    write!(stdout, "\n").unwrap();
    write!(stdout, "{RESET_CURSOR}").unwrap();
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
