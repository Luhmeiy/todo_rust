use std::io;

mod list;
mod task;

enum Command {
    Add(String),
    List,
    Update(usize, String),
    Complete(usize),
    Delete(usize),
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
            ["update", id_str, rest @ ..] if !rest.is_empty() => match id_str.parse() {
                Ok(id) => {
                    let task = rest.join(" ");
                    Ok(Command::Update(id, task))
                }
                Err(_) => Err("Invalid ID".to_string()),
            },
            ["complete", id_str] => match id_str.parse() {
                Ok(id) => Ok(Command::Complete(id)),
                Err(_) => Err("Invalid ID".to_string()),
            },
            ["delete", id_str] => match id_str.parse() {
                Ok(id) => Ok(Command::Delete(id)),
                Err(_) => Err("Invalid ID".to_string()),
            },
            [command, ..] => Err(format!("Invalid command: {command}")),
            [] => Err("Empty input".to_string()),
        }
    }

    fn execute(self, tasks: &mut list::TaskList) {
        match self {
            Command::Add(task) => tasks.add(task),
            Command::List => println!("List"),
            Command::Delete(id) => println!("Delete"),
            Command::Update(id, task) => println!("Update"),
            Command::Complete(id) => println!("Complete"),
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
