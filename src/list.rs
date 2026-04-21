use crate::task::{Priority, Task};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(PartialEq)]
pub enum TaskId {
    Number(usize),
    String(String),
}

#[derive(Debug)]
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

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn get_title(&self) -> &str {
        &self.title
    }

    fn is_not_empty<F: FnMut(&mut Vec<Task>)>(&mut self, mut f: F) -> Result<(), ListError> {
        if self.tasks.is_empty() {
            return Err(ListError::Empty);
        } else {
            f(&mut self.tasks);
            Ok(())
        }
    }

    pub fn rename(&mut self, title: String) -> Result<(String, &str), ListError> {
        let old_title = std::mem::replace(&mut self.title, title);
        Ok((old_title, &self.title))
    }

    pub fn add(&mut self, description: String) -> Result<&Task, ListError> {
        self.tasks.push(Task::new(description));
        Ok(self.tasks.last().unwrap())
    }

    pub fn list(&mut self) -> Result<(), ListError> {
        self.is_not_empty(|tasks| {
            for task in tasks.iter() {
                task.display();
            }
        })
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

    pub fn update(
        &mut self,
        query: TaskId,
        description: String,
    ) -> Result<(String, &str), ListError> {
        let id = self.resolve_index(query)?;
        let (old_description, new_description) = self.tasks[id].update(description);
        Ok((old_description, new_description))
    }

    pub fn get_due_date(&self, query: TaskId) -> Result<(&str, Option<NaiveDate>), ListError> {
        let id = self.resolve_index(query)?;
        let task = &self.tasks[id];

        let description = task.get_description();
        let due_date = task.get_due_date();

        Ok((description, due_date))
    }

    pub fn remove_due_date(&mut self, query: TaskId) -> Result<&str, ListError> {
        let id = self.resolve_index(query)?;
        let task = &mut self.tasks[id];
        task.remove_due_date();
        Ok(task.get_description())
    }

    pub fn add_due_date(&mut self, query: TaskId, date: NaiveDate) -> Result<&str, ListError> {
        let id = self.resolve_index(query)?;
        let task = &mut self.tasks[id];
        task.add_due_date(date);
        Ok(task.get_description())
    }

    pub fn get_priority(&self, query: TaskId) -> Result<(&str, Option<Priority>), ListError> {
        let id = self.resolve_index(query)?;
        let task = &self.tasks[id];

        let description = task.get_description();
        let priority = task.get_priority();

        Ok((description, priority))
    }

    pub fn remove_priority(&mut self, query: TaskId) -> Result<&str, ListError> {
        let id = self.resolve_index(query)?;
        let task = &mut self.tasks[id];
        task.remove_priority();
        Ok(task.get_description())
    }

    pub fn add_priority(&mut self, query: TaskId, priority: Priority) -> Result<&str, ListError> {
        let id = self.resolve_index(query)?;
        let task = &mut self.tasks[id];
        task.add_priority(priority);
        Ok(task.get_description())
    }

    pub fn check_all(&mut self) -> Result<(), ListError> {
        self.is_not_empty(|tasks| {
            for task in tasks.iter_mut() {
                task.check()
            }
        })
    }

    pub fn check(&mut self, query: TaskId) -> Result<&str, ListError> {
        let id = self.resolve_index(query)?;
        let task = &mut self.tasks[id];
        task.check();
        Ok(task.get_description())
    }

    pub fn uncheck_all(&mut self) -> Result<(), ListError> {
        self.is_not_empty(|tasks| {
            for task in tasks.iter_mut() {
                task.uncheck()
            }
        })
    }

    pub fn uncheck(&mut self, query: TaskId) -> Result<&str, ListError> {
        let id = self.resolve_index(query)?;
        let task = &mut self.tasks[id];
        task.uncheck();
        Ok(task.get_description())
    }

    pub fn delete_all(&mut self) -> Result<(), ListError> {
        self.is_not_empty(|tasks| tasks.clear())
    }

    pub fn delete_checked(&mut self) -> Result<(), ListError> {
        self.is_not_empty(|tasks| tasks.retain(|task| !task.is_checked()))
    }
    pub fn delete_unchecked(&mut self) -> Result<(), ListError> {
        self.is_not_empty(|tasks| tasks.retain(|task| task.is_checked()))
    }

    pub fn delete(&mut self, query: TaskId) -> Result<Task, ListError> {
        let id = self.resolve_index(query)?;
        let task = self.tasks.remove(id);
        Ok(task)
    }
}
