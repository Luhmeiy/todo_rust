use chrono::NaiveDate;
use colored::Colorize;
use std::path::PathBuf;

use crate::{
    command_info,
    config::Config,
    error::AppError,
    help,
    list::TaskId,
    manager::{ListId, ListManager},
    task::Priority,
};

pub enum Command {
    MakeList(String),
    Lists,
    Switch(ListId),
    RemoveList(ListId),
    Rename(usize, String),
    RenameCurrent(String),
    Add(String, Option<NaiveDate>, Option<Priority>),
    List,
    ListAll,
    ListFrom(ListId),
    Dues,
    Update(TaskId, String),
    DueView(TaskId),
    DueRemove(TaskId),
    DueAdd(TaskId, NaiveDate),
    PriorityView(TaskId),
    PriorityRemove(TaskId),
    PriorityAdd(TaskId, Priority),
    CheckAll,
    Check(TaskId),
    UncheckAll,
    Uncheck(TaskId),
    DeleteAll,
    DeleteChecked,
    DeleteUnchecked,
    Delete(TaskId),
    Save(String),
    Load(String),
    AliasAdd(String, String),
    AliasList,
    AliasRemove(String),
    AliasRename(String, String),
    AliasPath(String, String),
    Help(Option<String>),
    Exit,
}

fn allowed_when_empty(input: &str, list_manager: &ListManager) -> Result<(), String> {
    if list_manager.is_empty() {
        let first_word = input.split_whitespace().next().unwrap_or("");

        if command_info::requires_list(first_word) {
            return Err("A list is necessary to perform this action. Create a new list with \"mklist name of your list\"".to_string());
        }
    }

    Ok(())
}

fn parse_add_flags(args: &[&str]) -> Result<(String, Option<NaiveDate>, Option<Priority>), String> {
    let mut task_parts = Vec::new();
    let mut due_date = None;
    let mut priority = None;
    let mut i = 0;

    while i < args.len() {
        match args[i] {
            "--due" | "-d" => {
                if i + 1 >= args.len() {
                    return Err("Missing date after --due/-d flag".to_string());
                }

                let date_str = args[i + 1];
                let naive_date = NaiveDate::parse_from_str(date_str, "%d-%m-%Y")
                    .map_err(|_| "Invalid date format. Use DD-MM-YYYY".to_string())?;

                due_date = Some(naive_date);
                i += 2;
            }
            "--priority" | "-p" => {
                if i + 1 >= args.len() {
                    return Err("Missing priority level after --priority/-p flag".to_string());
                }

                let priority_str = args[i + 1].to_lowercase();
                priority = match priority_str.as_str() {
                    "low" => Some(Priority::Low),
                    "medium" => Some(Priority::Medium),
                    "high" => Some(Priority::High),
                    _ => return Err("Invalid priority. Use low, medium, or high.".to_string()),
                };

                i += 2;
            }
            arg => {
                task_parts.push(arg.to_string());
                i += 1;
            }
        }
    }

    let task = task_parts.join(" ");
    if task.is_empty() {
        return Err("add requires a task description.".to_string());
    }

    Ok((task, due_date, priority))
}

impl Command {
    pub fn parse_command(command: &str, list_manager: &ListManager) -> Result<Self, String> {
        allowed_when_empty(command, list_manager)?;
        let split_command: Vec<&str> = command.split_whitespace().collect();

        match split_command.as_slice() {
            ["mklist"] => Err("mklist requires a list title.".to_string()),
            ["mklist", rest @ ..] => {
                let list = rest.join(" ");
                Ok(Command::MakeList(list))
            }
            ["lists"] => Ok(Command::Lists),
            ["lists", _rest @ ..] => Err("lists takes no parameters.".to_string()),
            ["switch"] => Err("switch requires an ID or list title.".to_string()),
            ["switch", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::Switch(ListId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Ok(Command::Switch(ListId::String(query.join(" ")))),
            },
            ["rmlist"] => Err("rmlist requires an ID or list title.".to_string()),
            ["rmlist", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::RemoveList(ListId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Ok(Command::RemoveList(ListId::String(query.join(" ")))),
            },
            ["rename"] => Err("rename requires a new list title.".to_string()),
            ["rename", rest @ ..] => match rest.len() {
                0 => unreachable!(),
                1 => Ok(Command::RenameCurrent(rest[0].to_string())),
                _ => {
                    if let Ok(id) = rest[0].parse::<usize>() {
                        let title = rest[1..].join(" ");
                        Ok(Command::Rename(id - 1, title))
                    } else {
                        let title = rest.join(" ");
                        Ok(Command::RenameCurrent(title))
                    }
                }
            },
            ["add", rest @ ..] => {
                let (task, due_date, priority) = parse_add_flags(rest)?;
                Ok(Command::Add(task, due_date, priority))
            }
            ["list"] => Ok(Command::List),
            ["list", "--all"] => Ok(Command::ListAll),
            ["list", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::ListFrom(ListId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Ok(Command::ListFrom(ListId::String(query.join(" ")))),
            },
            ["dues"] => Ok(Command::Dues),
            ["dues", _rest @ ..] => Err("dues takes no parameters.".to_string()),
            ["update"] => Err("update requires an ID and a new task description.".to_string()),
            ["update", query] => match query.parse::<usize>() {
                Ok(id) if id > 0 => Err("update also requires a new task description.".to_string()),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Err("Invalid ID.".to_string()),
            },
            ["update", query, rest @ ..] => match query.parse::<usize>() {
                Ok(id) if id > 0 => {
                    let task = rest.join(" ");
                    Ok(Command::Update(TaskId::Number(id - 1), task))
                }
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Err("Invalid ID.".to_string()),
            },
            ["due"] => Err("due requires an ID and a due date.".to_string()),
            ["due", query] => match query.parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::DueView(TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Err("Invalid ID.".to_string()),
            },
            ["due", query, "--remove"] => match query.parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::DueRemove(TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Err("Invalid ID.".to_string()),
            },
            ["due", query, date_str] => match query.parse::<usize>() {
                Ok(id) if id > 0 => {
                    let naive_date = NaiveDate::parse_from_str(date_str, "%d-%m-%Y")
                        .map_err(|_| "Invalid date format. Use DD-MM-YYYY".to_string())?;
                    Ok(Command::DueAdd(TaskId::Number(id - 1), naive_date))
                }
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Err("Invalid ID.".to_string()),
            },
            ["priority"] => Err("priority requires an ID and a priority level.".to_string()),
            ["priority", query] => match query.parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::PriorityView(TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Err("Invalid ID.".to_string()),
            },
            ["priority", query, "--remove"] => match query.parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::PriorityRemove(TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Err("Invalid ID.".to_string()),
            },
            ["priority", query, priority_str] => match query.parse::<usize>() {
                Ok(id) if id > 0 => {
                    let priority = match priority_str.to_lowercase().as_str() {
                        "low" => Priority::Low,
                        "medium" => Priority::Medium,
                        "high" => Priority::High,
                        _ => {
                            return Err("Invalid priority. Use low, medium, or high."
                                .to_string()
                                .into());
                        }
                    };

                    Ok(Command::PriorityAdd(TaskId::Number(id - 1), priority))
                }
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Err("Invalid ID.".to_string()),
            },
            ["check"] => Err("check requires an ID or task description.".to_string()),
            ["check", "--all", _rest @ ..] => Ok(Command::CheckAll),
            ["check", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::Check(TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Ok(Command::Check(TaskId::String(query.join(" ")))),
            },
            ["uncheck"] => Err("uncheck requires an ID or task description.".to_string()),
            ["uncheck", "--all", _rest @ ..] => Ok(Command::UncheckAll),
            ["uncheck", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::Uncheck(TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Ok(Command::Uncheck(TaskId::String(query.join(" ")))),
            },
            ["delete"] => Err("delete requires an ID or task description.".to_string()),
            ["delete", "--all", _rest @ ..] => Ok(Command::DeleteAll),
            ["delete", "--checked", _rest @ ..] => Ok(Command::DeleteChecked),
            ["delete", "--unchecked", _rest @ ..] => Ok(Command::DeleteUnchecked),
            ["delete", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::Delete(TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Ok(Command::Delete(TaskId::String(query.join(" ")))),
            },
            ["save"] => Err("save requires a file path.".to_string()),
            ["save", path @ ..] => Ok(Command::Save(path.join(" "))),
            ["load"] => Err("load requires a file path.".to_string()),
            ["load", path @ ..] => Ok(Command::Load(path.join(" "))),
            ["alias"] => {
                Err("alias requires a subcommand (add, list, remove, rename, path).".to_string())
            }
            ["alias", "add"] => Err("alias add requires a name and path.".to_string()),
            ["alias", "add", _alias] => Err("alias add also requires a path.".to_string()),
            ["alias", "add", alias, path @ ..] => {
                Ok(Command::AliasAdd(alias.to_string(), path.join(" ")))
            }
            ["alias", "list"] => Ok(Command::AliasList),
            ["alias", "list", _rest @ ..] => Err("alias list takes no parameters.".to_string()),
            ["alias", "remove"] => Err("alias remove requires a name.".to_string()),
            ["alias", "remove", name @ ..] => Ok(Command::AliasRemove(name.join(" "))),
            ["alias", "rename"] => Err("alias rename requires old and new name.".to_string()),
            ["alias", "rename", _old] => Err("alias rename also requires a new name.".to_string()),
            ["alias", "rename", old_name, new_name] => Ok(Command::AliasRename(
                old_name.to_string(),
                new_name.to_string(),
            )),
            ["alias", "path"] => Err("alias path requires a name and path.".to_string()),
            ["alias", "path", _name] => Err("alias path also requires a path.".to_string()),
            ["alias", "path", name, path @ ..] => {
                Ok(Command::AliasPath(name.to_string(), path.join(" ")))
            }
            ["help"] => Ok(Command::Help(None)),
            ["help", command @ ..] => Ok(Command::Help(Some(command.join(" ")))),
            ["exit", _rest @ ..] => Ok(Command::Exit),
            [command, ..] => Err(format!("Invalid command: {command}")),
            [] => Err("Empty input.".to_string()),
        }
    }

    pub fn execute(
        self,
        list_manager: &mut ListManager,
        config: &mut Config,
    ) -> Result<(), AppError> {
        match self {
            Command::MakeList(list) => {
                let list = list_manager.add(list)?;
                println!("Created list {}", list.get_title().cyan())
            }
            Command::Lists => list_manager.list()?,
            Command::Switch(id) => {
                let title = list_manager.switch(id)?;
                println!("Switched to list {}", title.cyan())
            }
            Command::RemoveList(id) => {
                let list = list_manager.delete(id)?;
                println!("Removed list {}", list.get_title().cyan())
            }
            Command::Rename(id, title) => {
                let (old_title, new_title) = list_manager.rename_by_id(id, title)?;
                println!("Renamed list {} to {}", old_title.cyan(), new_title.cyan())
            }
            Command::RenameCurrent(title) => {
                let tasks = list_manager.get_current_list()?;
                let (old_title, new_title) = tasks.rename(title)?;
                println!("Renamed list {} to {}", old_title.cyan(), new_title.cyan())
            }
            Command::Add(task, due_date, priority) => {
                let tasks = list_manager.get_current_list()?;
                let task = tasks.add(task, due_date, priority)?;
                println!("Added task {}", task.get_description().cyan())
            }
            Command::List => {
                let tasks = list_manager.get_current_list()?;
                tasks.list()?
            }
            Command::ListAll => list_manager.list_tasks()?,
            Command::ListFrom(query) => list_manager.list_from(query)?,
            Command::Dues => list_manager.get_due_tasks()?,
            Command::Update(id, task) => {
                let tasks = list_manager.get_current_list()?;
                let (old_description, new_description) = tasks.update(id, task)?;
                println!(
                    "Renamed task {} to {}",
                    old_description.cyan(),
                    new_description.cyan()
                )
            }
            Command::DueView(id) => {
                let tasks = list_manager.get_current_list()?;
                let (description, due_date) = tasks.get_due_date(id)?;

                let result = match due_date {
                    Some(due_date) => {
                        format!(
                            "[{}] {}",
                            description.cyan(),
                            due_date.format("%d-%m-%Y").to_string().cyan()
                        )
                    }
                    None => format!("No due date for task {}", description.cyan()),
                };

                println!("{}", result)
            }
            Command::DueRemove(id) => {
                let tasks = list_manager.get_current_list()?;
                let description = tasks.remove_due_date(id)?;
                println!("Removed due date for task {}", description.cyan())
            }
            Command::DueAdd(id, date) => {
                let tasks = list_manager.get_current_list()?;
                let description = tasks.add_due_date(id, date)?;
                println!("Set due date for task {}", description.cyan())
            }
            Command::PriorityView(id) => {
                let tasks = list_manager.get_current_list()?;
                let (description, priority) = tasks.get_priority(id)?;

                let result = match priority {
                    Some(priority) => {
                        format!("[{}] {:?}", description.cyan(), priority)
                    }
                    None => format!("No priority for task {}", description.cyan()),
                };

                println!("{}", result)
            }
            Command::PriorityRemove(id) => {
                let tasks = list_manager.get_current_list()?;
                let description = tasks.remove_priority(id)?;
                println!("Removed priority for task {}", description.cyan())
            }
            Command::PriorityAdd(id, priority) => {
                let tasks = list_manager.get_current_list()?;
                let description = tasks.add_priority(id, priority)?;
                println!("Set priority for task {}", description.cyan())
            }
            Command::CheckAll => {
                let tasks = list_manager.get_current_list()?;
                tasks.check_all()?;
                println!("Checked all tasks")
            }
            Command::Check(id) => {
                let tasks = list_manager.get_current_list()?;
                let description = tasks.check(id)?;
                println!("Checked task {}", description.cyan())
            }
            Command::UncheckAll => {
                let tasks = list_manager.get_current_list()?;
                tasks.uncheck_all()?;
                println!("Unchecked all tasks")
            }
            Command::Uncheck(id) => {
                let tasks = list_manager.get_current_list()?;
                let description = tasks.uncheck(id)?;
                println!("Unchecked task {}", description.cyan())
            }
            Command::DeleteChecked => {
                let tasks = list_manager.get_current_list()?;
                tasks.delete_checked()?;
                println!("Deleted all checked tasks")
            }
            Command::DeleteUnchecked => {
                let tasks = list_manager.get_current_list()?;
                tasks.delete_unchecked()?;
                println!("Deleted all unchecked tasks")
            }
            Command::DeleteAll => {
                let tasks = list_manager.get_current_list()?;
                tasks.delete_all()?;
                println!("Deleted all tasks")
            }
            Command::Delete(id) => {
                let tasks = list_manager.get_current_list()?;
                let task = tasks.delete(id)?;
                println!("Deleted task {}", task.get_description().cyan())
            }
            Command::Save(path) => {
                let mut path_buf = PathBuf::from(path.trim());

                if path.starts_with("@") {
                    let path_from_alias = config.get_path_from_alias(&path)?;
                    path_buf = path_from_alias.to_path_buf();
                }

                let path = list_manager.save(Some(path_buf))?;

                config.change_path(PathBuf::from(path));
                config.save_with_warning();

                println!("Lists saved to {}", path.cyan())
            }
            Command::Load(path) => {
                let mut path_buf = PathBuf::from(path.trim());

                if path.starts_with("@") {
                    let path_from_alias = config.get_path_from_alias(&path)?;
                    path_buf = path_from_alias.to_path_buf();
                }

                *list_manager = ListManager::load(path_buf.clone())?;

                config.change_path(path_buf);
                config.save_with_warning();

                println!(
                    "Loaded lists from {}",
                    list_manager.get_path().to_string_lossy().cyan()
                )
            }
            Command::AliasAdd(alias, path) => {
                let alias = config.add_alias(alias, PathBuf::from(path))?;
                config.save_with_warning();
                println!("Added alias {}", alias.cyan())
            }
            Command::AliasList => config.list_alias()?,
            Command::AliasRemove(alias) => {
                let alias = config.remove_alias(alias)?;
                config.save_with_warning();
                println!("Removed alias {}", alias.cyan())
            }
            Command::AliasRename(old_name, new_name) => {
                let (old_name, new_name) = config.rename_alias(old_name, new_name)?;
                config.save_with_warning();
                println!("Renamed alias {} to {}", old_name.cyan(), new_name.cyan())
            }
            Command::AliasPath(name, new_path) => {
                let (name, new_path) = config.update_path_alias(name, new_path)?;
                config.save_with_warning();
                println!("Updated alias {} path to {}", name.cyan(), new_path.cyan())
            }
            Command::Help(None) => println!("{}", help::GENERAL.trim()),
            Command::Help(Some(command)) => {
                let help_text = if command.contains(' ') {
                    let parts: Vec<&str> = command.splitn(2, ' ').collect();
                    command_info::get_subcommand_help(parts[0], parts[1])
                } else {
                    command_info::get_help(&command)
                };

                match help_text {
                    Some(text) => println!("{}", text.trim()),
                    None => println!(
                        "No help available for '{command}'. Try 'help' for a list of commands."
                    ),
                }
            }
            Command::Exit => {
                println!("Exiting...");
                std::process::exit(0);
            }
        }
        Ok(())
    }

    pub fn is_mutation(&self) -> bool {
        matches!(
            self,
            Command::MakeList(_)
                | Command::Switch(_)
                | Command::RemoveList(_)
                | Command::Rename(_, _)
                | Command::RenameCurrent(_)
                | Command::Add(_, _, _)
                | Command::Update(_, _)
                | Command::DueRemove(_)
                | Command::DueAdd(_, _)
                | Command::PriorityRemove(_)
                | Command::PriorityAdd(_, _)
                | Command::CheckAll
                | Command::Check(_)
                | Command::UncheckAll
                | Command::Uncheck(_)
                | Command::DeleteAll
                | Command::DeleteChecked
                | Command::DeleteUnchecked
                | Command::Delete(_)
        )
    }

    pub fn is_alias_mutation(&self) -> bool {
        matches!(
            self,
            Command::AliasAdd(_, _)
                | Command::AliasRemove(_)
                | Command::AliasRename(_, _)
                | Command::AliasPath(_, _)
        )
    }
}
