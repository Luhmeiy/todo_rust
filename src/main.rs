use std::io;

mod command;
mod error;
mod list;
mod manager;
mod task;

fn main() {
    let mut list_manager = manager::ListManager::new();
    let _ = list_manager.add("Todo list".to_string());

    loop {
        let current_list_title = match list_manager.get_current_list() {
            Ok(list) => list.get_title().to_string(),
            Err(_) => String::new(),
        };

        println!("{current_list_title}");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        };

        match command::Command::parse_command(&input) {
            Ok(command) => {
                if let Err(error) = command.execute(&mut list_manager) {
                    eprintln!("Error: {error}");
                }
            }
            Err(error) => eprintln!("Error: {error}"),
        }
    }
}
