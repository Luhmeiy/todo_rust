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

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn display(&self) {
        let check = if self.checked { "x" } else { " " };
        println!("- [{check}] {}", self.description);
    }

    pub fn update(&mut self, description: String) {
        self.description = description
    }

    pub fn check(&mut self) {
        self.checked = true
    }

    pub fn uncheck(&mut self) {
        self.checked = false
    }
}
