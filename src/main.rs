use std::io;

mod command;
mod list;
mod task;

fn main() {
    let mut task_list = list::TaskList::new("Todo list".to_string());

    loop {
        println!("{}", task_list.get_title());

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        };

        match command::Command::parse_command(&input) {
            Ok(command) => {
                if let Err(error) = command.execute(&mut task_list) {
                    eprintln!("Error: {error}");
                }
            }
            Err(error) => eprintln!("Error: {error}"),
        }
    }
}
