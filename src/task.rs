use chrono::NaiveDate;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    description: String,
    checked: bool,
    created_at: NaiveDate,
    due_date: Option<NaiveDate>,
    priority: Option<Priority>,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            description,
            checked: false,
            created_at: NaiveDate::from(chrono::Local::now().date_naive()),
            due_date: None,
            priority: None,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_due_date(&self) -> Option<NaiveDate> {
        self.due_date
    }

    pub fn get_priority(&self) -> Option<Priority> {
        self.priority
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn display(&self) {
        let check = if self.checked { "✓" } else { " " };
        let priority = match self.priority {
            Some(priority) => format!("({priority:?})"),
            None => String::new(),
        };

        let display = format!("• [{}] {} {}", check, self.description, priority.cyan());

        if self.checked {
            println!("{}", display.green());
        } else {
            println!("{}", display);
        }
    }

    pub fn remove_due_date(&mut self) {
        self.due_date = None
    }

    pub fn add_due_date(&mut self, date: NaiveDate) {
        self.due_date = Some(date)
    }

    pub fn remove_priority(&mut self) {
        self.priority = None
    }

    pub fn add_priority(&mut self, priority: Priority) {
        self.priority = Some(priority)
    }

    pub fn update(&mut self, description: String) -> (String, &str) {
        let old_description = std::mem::replace(&mut self.description, description);
        (old_description, &self.description)
    }

    pub fn check(&mut self) {
        self.checked = true
    }

    pub fn uncheck(&mut self) {
        self.checked = false
    }
}
