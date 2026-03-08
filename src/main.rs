use std::io;

struct Task {
    description: String,
    completed: bool,
}

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

    fn execute(self, tasks: &mut Vec<Task>) {
        match self {
            Command::Add(task) => println!("Add"),
            Command::List => println!("List"),
            Command::Delete(id) => println!("Delete"),
            Command::Update(id, task) => println!("Update"),
            Command::Complete(id) => println!("Complete"),
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("Todo list");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        };

        match Command::parse_command(&input) {
            Ok(command) => command.execute(&mut tasks),
            Err(error) => eprintln!("Error: {error}"),
        }
    }
}
