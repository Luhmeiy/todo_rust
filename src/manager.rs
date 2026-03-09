use crate::list::TaskList;

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

    pub fn add(&mut self, title: String) -> Result<(), ManagerError> {
        self.lists.push(TaskList::new(title));
        Ok(())
    }

    pub fn list(&self) -> Result<(), ManagerError> {
        if self.lists.is_empty() {
            return Err(ManagerError::Empty);
        }

        for list in self.lists.iter() {
            println!("{}", list.get_title())
        }

        Ok(())
    }

    pub fn delete(&mut self, query: ListId) -> Result<(), ManagerError> {
        let id = match query {
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
        }?;

        self.lists.remove(id);
        Ok(())
    }
}
