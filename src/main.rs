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
            ["add", rest @ ..] if !rest.is_empty() => {
                let task = rest.join(" ");
                Ok(Command::Add(task))
            }
            ["list"] => Ok(Command::List),
            ["update", query, rest @ ..] if !rest.is_empty() => match query.parse::<usize>() {
                Ok(id) => {
                    let task = rest.join(" ");
                    Ok(Command::Update(list::TaskId::Number(id - 1), task))
                }
                Err(_) => Err("Invalid ID".to_string()),
            },
            ["check", "--all"] => Ok(Command::CheckAll),
            ["check", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) => Ok(Command::Check(list::TaskId::Number(id - 1))),
                Err(_) => Ok(Command::Check(list::TaskId::String(query.join(" ")))),
            },
            ["uncheck", "--all"] => Ok(Command::UncheckAll),
            ["uncheck", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) => Ok(Command::Uncheck(list::TaskId::Number(id - 1))),
                Err(_) => Ok(Command::Uncheck(list::TaskId::String(query.join(" ")))),
            },
            ["delete", "--all"] => Ok(Command::DeleteAll),
            ["delete", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) => Ok(Command::Delete(list::TaskId::Number(id - 1))),
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
