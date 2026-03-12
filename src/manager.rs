use crate::list::TaskList;
use colored::*;

#[derive(PartialEq)]
pub enum ListId {
    Number(usize),
    String(String),
}

pub enum ManagerError {
    Empty,
    NotFound,
    MultipleMatches(Vec<(usize, String)>),
}

impl std::fmt::Display for ManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManagerError::Empty => write!(
                f,
                "No lists found. Create a new list with \"mklist name of your list\""
            ),
            ManagerError::NotFound => write!(f, "List not found."),
            ManagerError::MultipleMatches(indices) => {
                writeln!(f, "Multiple matches. Please use the list number.")?;
                writeln!(f, "Matches:")?;

                for (id, description) in indices {
                    writeln!(f, "  {}. {}", id + 1, description)?;
                }
                Ok(())
            }
        }
    }
}

pub struct ListManager {
    lists: Vec<TaskList>,
    current_list: usize,
}

impl ListManager {
    pub fn new() -> Self {
        Self {
            lists: Vec::new(),
            current_list: 0,
        }
    }

    pub fn get_current_list(&mut self) -> Result<&mut TaskList, ManagerError> {
        if self.lists.is_empty() {
            return Err(ManagerError::Empty);
        }

        Ok(&mut self.lists[self.current_list])
    }

    pub fn rename_by_id(
        &mut self,
        id: usize,
        title: String,
    ) -> Result<(String, &str), ManagerError> {
        if id >= self.lists.len() {
            return Err(ManagerError::NotFound);
        }

        let (old_title, new_title) = self.lists[id].rename(title).unwrap();
        Ok((old_title, new_title))
    }

    pub fn add(&mut self, title: String) -> Result<&TaskList, ManagerError> {
        self.lists.push(TaskList::new(title));
        Ok(self.lists.last().unwrap())
    }

    pub fn list(&self) -> Result<(), ManagerError> {
        if self.lists.is_empty() {
            return Err(ManagerError::Empty);
        }

        for (index, list) in self.lists.iter().enumerate() {
            let current = if index == self.current_list {
                "[Current]".cyan().to_string()
            } else {
                "".to_string()
            };

            println!("{}. {} {current}", index + 1, list.get_title())
        }

        Ok(())
    }

    fn resolve_index(&self, query: ListId) -> Result<usize, ManagerError> {
        match query {
            ListId::Number(id) => {
                if id < self.lists.len() {
                    Ok(id)
                } else {
                    Err(ManagerError::NotFound)
                }
            }
            ListId::String(title) => {
                let matches: Vec<(usize, String)> = self
                    .lists
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.get_title() == &title)
                    .map(|(i, t)| (i, t.get_title().to_string()))
                    .collect();

                match matches.len() {
                    0 => Err(ManagerError::NotFound),
                    1 => Ok(matches[0].0),
                    _ => Err(ManagerError::MultipleMatches(matches)),
                }
            }
        }
    }

    pub fn switch(&mut self, query: ListId) -> Result<&str, ManagerError> {
        let id = self.resolve_index(query)?;
        self.current_list = id;
        Ok(self.lists[id].get_title())
    }

    pub fn delete(&mut self, query: ListId) -> Result<TaskList, ManagerError> {
        let id = self.resolve_index(query)?;
        self.current_list = self.current_list.saturating_sub(1);
        let task = self.lists.remove(id);
        Ok(task)
    }
}
