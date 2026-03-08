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

    pub fn update(&mut self, id: usize, description: String) {
        self.tasks[id].update(description)
    }

    pub fn check(&mut self, id: usize) {
        self.tasks[id].check()
    }

    pub fn uncheck(&mut self, id: usize) {
        self.tasks[id].uncheck()
    }

    pub fn delete(&mut self, id: usize) {
        self.tasks.remove(id);
    }
}
