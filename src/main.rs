use colored::Colorize;
use reedline::Signal;

mod command;
mod config;
mod editor;
mod error;
mod help;
mod list;
mod manager;
mod prompt;
mod task;

fn main() {
    let mut config = match config::Config::load() {
        Ok(loaded) => loaded,
        Err(_) => {
            let cfg = config::Config::new();
            if let Err(e) = cfg.save() {
                eprintln!("{} {e}", "Warning:".yellow())
            }
            cfg
        }
    };

    let mut list_manager = match manager::ListManager::load(config.get_path().to_path_buf()) {
        Ok(loaded) => loaded,
        Err(_) => manager::ListManager::new(),
    };

    if list_manager.is_empty() {
        let _ = list_manager.add("Todo list".to_string());
    }

    let mut list_exists = !list_manager.is_empty();
    let mut editor = editor::create_editor(list_exists, config.get_alias_names());
    let mut is_error = false;

    loop {
        let current_list_title = match list_manager.get_current_list() {
            Ok(list) => list.get_title().to_string(),
            Err(_) => String::new(),
        };

        let pointer = if is_error { ">".red() } else { ">".green() };
        is_error = false;

        let prompt = prompt::TodoPrompt::new(format!("{pointer} {} ", current_list_title.cyan()));

        match editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => {
                match command::Command::parse_command(&buffer, &mut list_manager) {
                    Ok(command) => {
                        let is_mutation = command.is_mutation();
                        let is_alias_mutation = command.is_alias_mutation();

                        if let Err(error) = command.execute(&mut list_manager, &mut config) {
                            is_error = true;
                            eprintln!("{} {error}", "Error:".red());
                        } else if is_mutation {
                            if let Err(error) = list_manager.save(None) {
                                eprintln!("{} {error}", "Warning:".yellow());
                            }

                            let new_list_exists = !list_manager.is_empty();
                            if new_list_exists != list_exists {
                                list_exists = new_list_exists;
                                editor =
                                    editor::create_editor(list_exists, config.get_alias_names());
                            }
                        } else if is_alias_mutation {
                            editor = editor::create_editor(list_exists, config.get_alias_names());
                        }
                    }
                    Err(error) => {
                        is_error = true;
                        eprintln!("{} {error}", "Error:".red())
                    }
                }
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("Exiting...");
                break;
            }
            Err(_) => {
                eprintln!("{} Failed to read input", "Error:".red());
                continue;
            }
        }

        println!()
    }
}
