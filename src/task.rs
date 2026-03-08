pub struct Task {
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            description,
            completed: false,
        }
    }

    pub fn display(&self) {
        let check = if self.completed { "x" } else { " " };
        println!("- [{check}] {}", self.description);
    }
}
