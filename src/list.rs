use crate::task::Task;

#[derive(PartialEq)]
pub enum TaskId {
    Number(usize),
    String(String),
}

pub enum LookupError {
    Empty,
    NotFound,
    MultipleMatches(Vec<usize>),
}

pub struct TaskList {
    title: String,
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new(title: String) -> Self {
        Self {
            title,
            tasks: Vec::new(),
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn add(&mut self, description: String) {
        self.tasks.push(Task::new(description));
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            self.handle_errors(LookupError::Empty);
            return;
        }

        for task in self.tasks.iter() {
            task.display();
        }
    }

    fn handle_errors(&self, error: LookupError) {
        match error {
            LookupError::Empty => println!("No tasks found."),
            LookupError::NotFound => println!("Task not found."),
            LookupError::MultipleMatches(indices) => {
                println!("Multiple matches. Please use the task number.");
                println!("Matches:");

                for id in indices {
                    println!("  {}. {}", id + 1, self.tasks[id].get_description());
                }
            }
        }
    }

    fn resolve_index(&self, query: TaskId) -> Result<usize, LookupError> {
        match query {
            TaskId::Number(id) => {
                if id < self.tasks.len() {
                    Ok(id)
                } else {
                    Err(LookupError::NotFound)
                }
            }
            TaskId::String(description) => {
                let matches: Vec<usize> = self
                    .tasks
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.get_description() == &description)
                    .map(|(i, _)| i)
                    .collect();

                match matches.len() {
                    0 => Err(LookupError::NotFound),
                    1 => Ok(matches[0]),
                    _ => Err(LookupError::MultipleMatches(matches)),
                }
            }
        }
    }

    fn validate_index<T: FnOnce(&mut Task)>(&mut self, query: TaskId, f: T) {
        match self.resolve_index(query) {
            Ok(id) => f(&mut self.tasks[id]),
            Err(error) => self.handle_errors(error),
        };
    }

    pub fn update(&mut self, query: TaskId, description: String) {
        self.validate_index(query, |t| t.update(description));
    }

    fn is_not_empty<F: FnMut(&mut Task)>(&mut self, mut f: F) {
        if self.tasks.is_empty() {
            self.handle_errors(LookupError::Empty);
        } else {
            for task in self.tasks.iter_mut() {
                f(task)
            }
        }
    }

    pub fn check_all(&mut self) {
        self.is_not_empty(|task| task.check())
    }

    pub fn check(&mut self, query: TaskId) {
        self.validate_index(query, |t| t.check());
    }

    pub fn uncheck_all(&mut self) {
        self.is_not_empty(|task| task.uncheck())
    }

    pub fn uncheck(&mut self, query: TaskId) {
        self.validate_index(query, |t| t.uncheck());
    }

    pub fn delete_all(&mut self) {
        if self.tasks.is_empty() {
            self.handle_errors(LookupError::Empty);
            return;
        }

        self.tasks.clear()
    }

    pub fn delete(&mut self, query: TaskId) {
        match self.resolve_index(query) {
            Ok(id) => {
                self.tasks.remove(id);
            }
            Err(error) => self.handle_errors(error),
        }
    }
}
