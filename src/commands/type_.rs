use crate::commands::commands::Commands;
use crate::util::files::find_in_path;

#[inline(always)]
pub fn handle_type(args: &mut Vec<String>) {
    let to_check = Commands::from_str(args.pop().unwrap().as_str());
    match to_check {
        Commands::Other(file) => match find_in_path(&file) {
            Some(file_path) => println!("{file} is {file_path}"),
            None => println!("{file}: not found"),
        },
        _ => println!("{} is a shell builtin", to_check.as_str()),
    }
}
