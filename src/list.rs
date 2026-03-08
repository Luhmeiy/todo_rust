use crate::task::Task;

pub struct TaskList {
    title: String,
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new(title: String) -> Self {
        Self {
            title,
            tasks: Vec::new(),
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn add(&mut self, description: String) {
        self.tasks.push(Task::new(description));
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        for task in self.tasks.iter() {
            task.display();
        }
    }

    fn validate_index<T: FnOnce(&mut Task)>(&mut self, id: usize, f: T) {
        match self.tasks.get_mut(id) {
            Some(task) => f(task),
            None => println!("Invalid ID."),
        };
    }

    pub fn update(&mut self, id: usize, description: String) {
        self.validate_index(id, |t| t.update(description));
    }

    pub fn check(&mut self, id: usize) {
        self.validate_index(id, |t| t.check());
    }

    pub fn uncheck(&mut self, id: usize) {
        self.validate_index(id, |t| t.uncheck());
    }

    pub fn delete(&mut self, id: usize) {
        if id >= self.tasks.len() {
            println!("Invalid ID.");
            return;
        }

        self.tasks.remove(id);
    }
}
