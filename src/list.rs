use crate::task::Task;

#[derive(PartialEq)]
pub enum TaskId {
    Number(usize),
    String(String),
}

pub enum ListError {
    Empty,
    NotFound,
    MultipleMatches(Vec<(usize, String)>),
}

impl std::fmt::Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListError::Empty => write!(f, "No tasks found."),
            ListError::NotFound => write!(f, "Task not found."),
            ListError::MultipleMatches(indices) => {
                writeln!(f, "Multiple matches. Please use the task number.")?;
                writeln!(f, "Matches:")?;

                for (id, description) in indices {
                    writeln!(f, "  {}. {}", id + 1, description)?;
                }
                Ok(())
            }
        }
    }
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

    pub fn add(&mut self, description: String) -> Result<(), ListError> {
        self.tasks.push(Task::new(description));
        Ok(())
    }

    pub fn list(&self) -> Result<(), ListError> {
        if self.tasks.is_empty() {
            return Err(ListError::Empty);
        }

        for task in self.tasks.iter() {
            task.display();
        }

        Ok(())
    }

    fn resolve_index(&self, query: TaskId) -> Result<usize, ListError> {
        match query {
            TaskId::Number(id) => {
                if id < self.tasks.len() {
                    Ok(id)
                } else {
                    Err(ListError::NotFound)
                }
            }
            TaskId::String(description) => {
                let matches: Vec<(usize, String)> = self
                    .tasks
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.get_description() == &description)
                    .map(|(i, t)| (i, t.get_description().to_string()))
                    .collect();

                match matches.len() {
                    0 => Err(ListError::NotFound),
                    1 => Ok(matches[0].0),
                    _ => Err(ListError::MultipleMatches(matches)),
                }
            }
        }
    }

    pub fn update(&mut self, query: TaskId, description: String) -> Result<(), ListError> {
        let id = self.resolve_index(query)?;
        self.tasks[id].update(description);
        Ok(())
    }

    fn is_not_empty<F: FnMut(&mut Task)>(&mut self, mut f: F) -> Result<(), ListError> {
        if self.tasks.is_empty() {
            return Err(ListError::Empty);
        } else {
            for task in self.tasks.iter_mut() {
                f(task)
            }

            Ok(())
        }
    }

    pub fn check_all(&mut self) -> Result<(), ListError> {
        self.is_not_empty(|task| task.check())
    }

    pub fn check(&mut self, query: TaskId) -> Result<(), ListError> {
        let id = self.resolve_index(query)?;
        self.tasks[id].check();
        Ok(())
    }

    pub fn uncheck_all(&mut self) -> Result<(), ListError> {
        self.is_not_empty(|task| task.uncheck())
    }

    pub fn uncheck(&mut self, query: TaskId) -> Result<(), ListError> {
        let id = self.resolve_index(query)?;
        self.tasks[id].uncheck();
        Ok(())
    }

    pub fn delete_all(&mut self) -> Result<(), ListError> {
        if self.tasks.is_empty() {
            return Err(ListError::Empty);
        }

        self.tasks.clear();
        Ok(())
    }

    pub fn delete(&mut self, query: TaskId) -> Result<(), ListError> {
        let id = self.resolve_index(query)?;
        self.tasks.remove(id);
        Ok(())
    }
}
