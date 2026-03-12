use crate::{
    error::AppError,
    help,
    list::TaskId,
    manager::{ListId, ListManager},
};

pub enum Command {
    MakeList(String),
    Lists,
    Switch(ListId),
    RemoveList(ListId),
    Rename(usize, String),
    RenameCurrent(String),
    Add(String),
    List,
    Update(TaskId, String),
    CheckAll,
    Check(TaskId),
    UncheckAll,
    Uncheck(TaskId),
    DeleteAll,
    DeleteChecked,
    DeleteUnchecked,
    Delete(TaskId),
    Help(Option<String>),
    Exit,
}

impl Command {
    pub fn parse_command(command: &str) -> Result<Self, String> {
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
            ["add"] => Err("add requires a task description.".to_string()),
            ["add", rest @ ..] => {
                let task = rest.join(" ");
                Ok(Command::Add(task))
            }
            ["list"] => Ok(Command::List),
            ["list", _rest @ ..] => Err("list takes no parameters.".to_string()),
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
            ["help"] => Ok(Command::Help(None)),
            ["help", command @ ..] => Ok(Command::Help(Some(command.join(" ")))),
            ["exit", _rest @ ..] => Ok(Command::Exit),
            [command, ..] => Err(format!("Invalid command: {command}")),
            [] => Err("Empty input.".to_string()),
        }
    }

    pub fn execute(self, list_manager: &mut ListManager) -> Result<(), AppError> {
        match self {
            Command::MakeList(list) => list_manager.add(list)?,
            Command::Lists => list_manager.list()?,
            Command::Switch(id) => list_manager.switch(id)?,
            Command::RemoveList(id) => list_manager.delete(id)?,
            Command::Rename(id, title) => list_manager.rename_by_id(id, title)?,
            Command::RenameCurrent(title) => {
                let tasks = list_manager.get_current_list()?;
                tasks.rename(title)?
            }
            Command::Add(task) => {
                let tasks = list_manager.get_current_list()?;
                tasks.add(task)?
            }
            Command::List => {
                let tasks = list_manager.get_current_list()?;
                tasks.list()?
            }
            Command::Update(id, task) => {
                let tasks = list_manager.get_current_list()?;
                tasks.update(id, task)?
            }
            Command::CheckAll => {
                let tasks = list_manager.get_current_list()?;
                tasks.check_all()?
            }
            Command::Check(id) => {
                let tasks = list_manager.get_current_list()?;
                tasks.check(id)?
            }
            Command::UncheckAll => {
                let tasks = list_manager.get_current_list()?;
                tasks.uncheck_all()?
            }
            Command::Uncheck(id) => {
                let tasks = list_manager.get_current_list()?;
                tasks.uncheck(id)?
            }
            Command::DeleteChecked => {
                let tasks = list_manager.get_current_list()?;
                tasks.delete_checked()?
            }
            Command::DeleteUnchecked => {
                let tasks = list_manager.get_current_list()?;
                tasks.delete_unchecked()?
            }
            Command::DeleteAll => {
                let tasks = list_manager.get_current_list()?;
                tasks.delete_all()?
            }
            Command::Delete(id) => {
                let tasks = list_manager.get_current_list()?;
                tasks.delete(id)?
            }
            Command::Help(None) => println!("{}", help::GENERAL.trim()),
            Command::Help(Some(command)) => match help::for_command(&command) {
                Some(text) => println!("{}", text.trim()),
                None => println!(
                    "No help available for '{command}'. Try 'help' for a list of commands."
                ),
            },
            Command::Exit => {
                println!("Exiting...");
                std::process::exit(0);
            }
        }
        Ok(())
    }
}
