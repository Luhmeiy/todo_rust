use crate::{error::AppError, list::TaskId, manager::ListManager};

pub enum Command {
    MakeList(String),
    Add(String),
    List,
    Update(TaskId, String),
    CheckAll,
    Check(TaskId),
    UncheckAll,
    Uncheck(TaskId),
    DeleteAll,
    Delete(TaskId),
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
            ["delete", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::Delete(TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer.".to_string()),
                Err(_) => Ok(Command::Delete(TaskId::String(query.join(" ")))),
            },
            [command, ..] => Err(format!("Invalid command: {command}")),
            [] => Err("Empty input.".to_string()),
        }
    }

    pub fn execute(self, list_manager: &mut ListManager) -> Result<(), AppError> {
        match self {
            Command::MakeList(list) => list_manager.add(list)?,
            Command::Add(task) => {
                let tasks = list_manager.get_current_list();
                tasks.add(task)?
            }
            Command::List => {
                let tasks = list_manager.get_current_list();
                tasks.list()?
            }
            Command::Update(id, task) => {
                let tasks = list_manager.get_current_list();
                tasks.update(id, task)?
            }
            Command::CheckAll => {
                let tasks = list_manager.get_current_list();
                tasks.check_all()?
            }
            Command::Check(id) => {
                let tasks = list_manager.get_current_list();
                tasks.check(id)?
            }
            Command::UncheckAll => {
                let tasks = list_manager.get_current_list();
                tasks.uncheck_all()?
            }
            Command::Uncheck(id) => {
                let tasks = list_manager.get_current_list();
                tasks.uncheck(id)?
            }
            Command::DeleteAll => {
                let tasks = list_manager.get_current_list();
                tasks.delete_all()?
            }
            Command::Delete(id) => {
                let tasks = list_manager.get_current_list();
                tasks.delete(id)?
            }
        }
        Ok(())
    }
}
