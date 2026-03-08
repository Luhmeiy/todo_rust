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
}
