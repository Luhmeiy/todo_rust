use colored::*;

pub struct Task {
    description: String,
    checked: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            description,
            checked: false,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
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
