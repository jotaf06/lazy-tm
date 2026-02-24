use crate::task::Task;

#[derive(Default)]
pub struct App {
    pub tasks: Vec<Task>,
    pub next_id: u64,
}

impl App {
    pub fn new(tasks: Vec<Task>) -> Self {
        let next_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);

        Self { tasks, next_id }
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

    pub fn edit_task(&mut self, id: u64, title: String, description: String) {
        for task in &mut self.tasks {
            if task.id == id {
                task.title = title;
                task.description = description;
                break;
            }
        }
    }

    pub fn toggle_task(&mut self, id: u64) {
        for task in &mut self.tasks {
            if task.id == id {
                task.is_checked = !task.is_checked;
                break;
            }
        }
    }

    pub fn clear_all_tasks(&mut self) {
        self.tasks.clear();
        self.next_id = 0;
    }
}
