use chrono::NaiveDate;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    description: String,
    checked: bool,
    created_at: NaiveDate,
    due_date: Option<NaiveDate>,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            description,
            checked: false,
            created_at: NaiveDate::from(chrono::Local::now().date_naive()),
            due_date: None,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_due_date(&self) -> Option<NaiveDate> {
        self.due_date
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn display(&self) {
        let check = if self.checked { "✓" } else { " " };
        let display = format!("• [{}] {}", check, self.description);

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
