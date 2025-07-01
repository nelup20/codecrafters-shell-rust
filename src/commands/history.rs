use crate::commands::command::Command;

#[inline(always)]
pub fn handle_history(history: &Vec<String>) {
    for (i, line) in history.iter().enumerate() {
        println!("{} {}", i + 1, line)
    }
}

pub fn add_to_history(history: &mut Vec<String>, command: &Command) {
    let mut line = String::new();
    line.push_str(command.command_type.as_str());
    line.push_str(" ");
    line.push_str(&command.args.join(" "));
    
    history.push(line);
}
