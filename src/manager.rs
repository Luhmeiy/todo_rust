use crate::list::TaskList;

pub enum ManagerError {
    Empty,
    NotFound,
    MultipleMatches(Vec<(usize, String)>),
}

impl std::fmt::Display for ManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManagerError::Empty => write!(f, "No lists found."),
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

    pub fn get_current_list(&mut self) -> &mut TaskList {
        &mut self.lists[self.current_list]
    }

    pub fn add(&mut self, title: String) -> Result<(), ManagerError> {
        self.lists.push(TaskList::new(title));
        Ok(())
    }
}
