use std::io;

mod list;
mod task;

enum Command {
    Add(String),
    List,
    Update(list::TaskId, String),
    CheckAll,
    Check(list::TaskId),
    UncheckAll,
    Uncheck(list::TaskId),
    DeleteAll,
    Delete(list::TaskId),
}

impl Command {
    fn parse_command(command: &str) -> Result<Self, String> {
        let split_command: Vec<&str> = command.split_whitespace().collect();

        match split_command.as_slice() {
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
                Ok(_) => Err("ID must be a positive integer".to_string()),
                Err(_) => Err("Invalid ID".to_string()),
            },
            ["update", query, rest @ ..] => match query.parse::<usize>() {
                Ok(id) if id > 0 => {
                    let task = rest.join(" ");
                    Ok(Command::Update(list::TaskId::Number(id - 1), task))
                }
                Ok(_) => Err("ID must be a positive integer".to_string()),
                Err(_) => Err("Invalid ID".to_string()),
            },
            ["check"] => Err("check requires an ID or task description.".to_string()),
            ["check", "--all", _rest @ ..] => Ok(Command::CheckAll),
            ["check", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::Check(list::TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer".to_string()),
                Err(_) => Ok(Command::Check(list::TaskId::String(query.join(" ")))),
            },
            ["uncheck"] => Err("uncheck requires an ID or task description.".to_string()),
            ["uncheck", "--all", _rest @ ..] => Ok(Command::UncheckAll),
            ["uncheck", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::Uncheck(list::TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer".to_string()),
                Err(_) => Ok(Command::Uncheck(list::TaskId::String(query.join(" ")))),
            },
            ["delete"] => Err("delete requires an ID or task description.".to_string()),
            ["delete", "--all", _rest @ ..] => Ok(Command::DeleteAll),
            ["delete", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) if id > 0 => Ok(Command::Delete(list::TaskId::Number(id - 1))),
                Ok(_) => Err("ID must be a positive integer".to_string()),
                Err(_) => Ok(Command::Delete(list::TaskId::String(query.join(" ")))),
            },
            [command, ..] => Err(format!("Invalid command: {command}")),
            [] => Err("Empty input".to_string()),
        }
    }

    fn execute(self, tasks: &mut list::TaskList) {
        let result = match self {
            Command::Add(task) => tasks.add(task),
            Command::List => tasks.list(),
            Command::Update(id, task) => tasks.update(id, task),
            Command::CheckAll => tasks.check_all(),
            Command::Check(id) => tasks.check(id),
            Command::UncheckAll => tasks.uncheck_all(),
            Command::Uncheck(id) => tasks.uncheck(id),
            Command::DeleteAll => tasks.delete_all(),
            Command::Delete(id) => tasks.delete(id),
        };

        if let Err(error) = result {
            eprintln!("{error}");
        }
    }
}

fn main() {
    let mut task_list = list::TaskList::new("Todo list".to_string());

    loop {
        println!("{}", task_list.get_title());

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        };

        match Command::parse_command(&input) {
            Ok(command) => command.execute(&mut task_list),
            Err(error) => eprintln!("Error: {error}"),
        }
    }
}
