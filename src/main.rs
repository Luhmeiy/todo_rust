use colored::Colorize;
use std::io::{self, Write};

mod command;
mod error;
mod help;
mod list;
mod manager;
mod task;

fn main() {
    let mut list_manager = manager::ListManager::new();
    let _ = list_manager.add("Todo list".to_string());
    let mut is_error = false;

    loop {
        let current_list_title = match list_manager.get_current_list() {
            Ok(list) => list.get_title().to_string(),
            Err(_) => String::new(),
        };

        let pointer = if is_error { ">".red() } else { ">".green() };
        is_error = false;

        print!("{pointer} {} ", current_list_title.cyan());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("{} Failed to read input", "Error:".red());
            continue;
        };

        match command::Command::parse_command(&input) {
            Ok(command) => {
                if let Err(error) = command.execute(&mut list_manager) {
                    is_error = true;
                    eprintln!("{} {error}", "Error:".red());
                }
            }
            Err(error) => {
                is_error = true;
                eprintln!("{} {error}", "Error:".red())
            }
        }

        println!()
    }
}
