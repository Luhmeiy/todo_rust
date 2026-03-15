use colored::Colorize;
use std::io::{self, Write};

mod command;
mod error;
mod help;
mod list;
mod manager;
mod task;

fn main() {
    let mut list_manager = match manager::ListManager::load(None) {
        Ok(loaded) => loaded,
        Err(_) => manager::ListManager::new(),
    };

    if list_manager.is_empty() {
        let _ = list_manager.add("Todo list".to_string());
    }

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

        match command::Command::parse_command(&input, &mut list_manager) {
            Ok(command) => {
                let is_mutation = command.is_mutation();

                if let Err(error) = command.execute(&mut list_manager) {
                    is_error = true;
                    eprintln!("{} {error}", "Error:".red());
                } else if is_mutation {
                    if let Err(error) = list_manager.save(None) {
                        eprintln!("{} {error}", "Warning:".yellow());
                    }
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
