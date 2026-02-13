use crate::task::Task;

pub struct App {
    tasks: Vec<Task>,
    next_id: u64,
}

impl App {
    pub fn new() -> App {
        App {
            tasks: Vec::new(),
            next_id: 0,
        }
    }

    pub fn generate_task_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    pub fn add_task(&mut self, title: String, description: String) {
        let id = self.generate_task_id();
        let task = Task::new(id, title, description);

        self.tasks.push(task);
    }

    pub fn toggle_task(&mut self, id: u64) {
        for task in &mut self.tasks {
            if task.id == id {
                task.is_checked = !task.is_checked;
                break;
            }
        }
    }

    pub fn remove_task(&mut self, id: u64) {
        for index in 0..self.tasks.len() {
            if self.tasks[index].id == id {
                self.tasks.remove(index);
                break;
            }
        }
    }

    pub fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }
}