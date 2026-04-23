use crate::config::Config;
use crate::history::History;
use crate::list::{ListError, ListFilter, TaskList};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(PartialEq)]
pub enum ListId {
    Number(usize),
    String(String),
}

pub enum ManagerError {
    Empty,
    NoTasks,
    NotFound,
    MultipleMatches(Vec<(usize, String)>),
    NoMatches,
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    InvalidFileFormat,
    NoHistory,
    ListError(ListError),
}

impl From<ListError> for ManagerError {
    fn from(e: ListError) -> Self {
        ManagerError::ListError(e)
    }
}

impl std::fmt::Display for ManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManagerError::Empty => write!(
                f,
                "No lists found. Create a new list with \"mklist name of your list\""
            ),
            ManagerError::NoTasks => write!(f, "No tasks found."),
            ManagerError::NotFound => write!(f, "List not found."),
            ManagerError::MultipleMatches(indices) => {
                writeln!(f, "Multiple matches. Please use the list number.")?;
                writeln!(f, "Matches:")?;

                for (id, description) in indices {
                    writeln!(f, "  {}. {}", id + 1, description)?;
                }
                Ok(())
            }
            ManagerError::NoMatches => write!(f, "No tasks match the filter."),
            ManagerError::IoError(e) => write!(f, "IO error: {e}"),
            ManagerError::JsonError(e) => write!(f, "JSON error: {e}"),
            ManagerError::InvalidFileFormat => {
                write!(f, "Invalid file format. Use .json extension.")
            }
            ManagerError::NoHistory => write!(f, "Nothing to undo."),
            ManagerError::ListError(e) => e.fmt(f),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListManager {
    lists: Vec<TaskList>,
    current_list: usize,
    path: PathBuf,
    #[serde(skip)]
    history: Option<History>,
}

impl ListManager {
    pub fn new() -> Self {
        Self {
            lists: Vec::new(),
            current_list: 0,
            path: PathBuf::from("./todo_data.json"),
            history: None,
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn get_current_list(&mut self) -> Result<&mut TaskList, ManagerError> {
        if self.lists.is_empty() {
            return Err(ManagerError::Empty);
        }

        Ok(&mut self.lists[self.current_list])
    }

    pub fn is_empty(&self) -> bool {
        self.lists.is_empty()
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

    pub fn get_due_tasks(&mut self, config: &Config) -> Result<(), ManagerError> {
        if self.lists.is_empty() {
            return Err(ManagerError::Empty);
        }

        let mut dues = Vec::new();

        for list in self.lists.iter() {
            dues.append(&mut list.get_due_tasks());
        }

        if dues.is_empty() {
            return Err(ManagerError::NoTasks);
        }

        dues.sort_by(|a, b| a.get_due_date().unwrap().cmp(&b.get_due_date().unwrap()));

        for due in dues {
            due.display_due(config)
        }

        Ok(())
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

    pub fn list_with_filter(&self, filter: &ListFilter) -> Result<(), ManagerError> {
        let mut has_matches = false;

        if self.lists.is_empty() {
            return Err(ManagerError::NoTasks);
        }

        for (index, list) in self.lists.iter().enumerate() {
            if !list.has_tasks() {
                continue;
            }

            let filtered = list.get_filtered_tasks(&filter);

            if !filtered.is_empty() {
                has_matches = true;

                if index != 0 {
                    println!()
                }

                println!("List: {}", list.get_title().cyan());

                for task in filtered {
                    task.display();
                }
            }
        }

        if !has_matches {
            return Err(ManagerError::NoMatches);
        }

        Ok(())
    }

    pub fn list_from(&self, query: ListId, filter: &ListFilter) -> Result<(), ManagerError> {
        let id = self.resolve_index(query)?;
        self.lists[id].list_with_filter(&filter)?;
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

    pub fn save(&mut self, path: Option<PathBuf>) -> Result<&str, ManagerError> {
        let path = path.unwrap_or(self.path.clone());

        if !path.to_string_lossy().ends_with(".json") {
            return Err(ManagerError::InvalidFileFormat);
        }

        if path != self.path {
            self.path = path
        }

        let json = serde_json::to_string_pretty(self).map_err(ManagerError::JsonError)?;
        fs::write(&self.path, json).map_err(ManagerError::IoError)?;
        Ok(self.path.to_str().unwrap())
    }

    pub fn load(path: PathBuf) -> Result<Self, ManagerError> {
        if !path.to_string_lossy().ends_with(".json") {
            return Err(ManagerError::InvalidFileFormat);
        }

        let content = fs::read_to_string(path).map_err(ManagerError::IoError)?;
        serde_json::from_str(&content).map_err(ManagerError::JsonError)
    }

    pub fn push_history(&mut self) {
        if self.lists.is_empty() {
            return;
        }

        if let Ok(json) = serde_json::to_string(self) {
            self.history = Some(History::new(json));
        }
    }

    pub fn undo(&mut self) -> Result<(), ManagerError> {
        let state = self.history.take().ok_or(ManagerError::NoHistory)?.pop();

        let restored: ListManager =
            serde_json::from_str(&state).map_err(ManagerError::JsonError)?;

        self.lists = restored.lists;
        self.current_list = restored.current_list;
        self.path = restored.path;

        Ok(())
    }
}
