use crate::list::{LookupError, TaskList};

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

    pub fn add(&mut self, title: String) -> Result<(), LookupError> {
        self.lists.push(TaskList::new(title));
        Ok(())
    }
}
