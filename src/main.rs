use std::io;

mod list;
mod task;

enum Command {
    Add(String),
    List,
    Update(list::TaskId, String),
    Check(list::TaskId),
    Uncheck(list::TaskId),
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
            ["check", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) => Ok(Command::Check(list::TaskId::Number(id - 1))),
                Err(_) => Ok(Command::Check(list::TaskId::String(query.join(" ")))),
            },
            ["uncheck", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) => Ok(Command::Uncheck(list::TaskId::Number(id - 1))),
                Err(_) => Ok(Command::Uncheck(list::TaskId::String(query.join(" ")))),
            },
            ["delete", query @ ..] => match query[0].parse::<usize>() {
                Ok(id) => Ok(Command::Delete(list::TaskId::Number(id - 1))),
                Err(_) => Ok(Command::Delete(list::TaskId::String(query.join(" ")))),
            },
            [command, ..] => Err(format!("Invalid command: {command}")),
            [] => Err("Empty input".to_string()),
        }
    }

    fn execute(self, tasks: &mut list::TaskList) {
        match self {
            Command::Add(task) => tasks.add(task),
            Command::List => tasks.list(),
            Command::Update(id, task) => tasks.update(id, task),
            Command::Check(id) => tasks.check(id),
            Command::Uncheck(id) => tasks.uncheck(id),
            Command::Delete(id) => tasks.delete(id),
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
