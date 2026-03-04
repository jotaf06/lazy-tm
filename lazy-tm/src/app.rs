use crate::task::Task;

pub struct TaskRepository {
    tasks: Vec<Task>,
}

impl TaskRepository {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }

    pub fn all(&self) -> &[Task] {
        &self.tasks
    }

    pub fn all_mut(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }

    pub fn find_by_id(&self, id: u64) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn find_by_id_mut(&mut self, id: u64) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn find_by_index(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    pub fn find_by_index_mut(&mut self, index: usize) -> Option<&mut Task> {
        self.tasks.get_mut(index)
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove(&mut self, index: usize) -> Option<Task> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    pub fn remove_by_id(&mut self, id: u64) -> Option<Task> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            Some(self.tasks.remove(pos))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }

    pub fn count(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn filter<F>(&self, predicate: F) -> Vec<&Task>
    where
        F: Fn(&Task) -> bool,
    {
        self.tasks.iter().filter(|t| predicate(t)).collect()
    }

    pub fn completed(&self) -> Vec<&Task> {
        self.filter(|t| t.is_checked)
    }

    pub fn pending(&self) -> Vec<&Task> {
        self.filter(|t| !t.is_checked)
    }

    pub fn running(&self) -> Vec<&Task> {
        self.filter(|t| t.is_running())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Task> {
        self.tasks.iter_mut()
    }
}

pub struct App {
    repository: TaskRepository,
    pub next_id: u64,
    pub tasks: Vec<Task>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            repository: TaskRepository::new(Vec::new()),
            next_id: 0,
            tasks: Vec::new(),
        }
    }
}

impl App {
    pub fn new(tasks: Vec<Task>) -> Self {
        let next_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);

        Self {
            repository: TaskRepository::new(tasks.clone()),
            next_id,
            tasks,
        }
    }

    pub fn empty() -> Self {
        Self {
            repository: TaskRepository::new(Vec::new()),
            next_id: 0,
            tasks: Vec::new(),
        }
    }

    fn sync(&mut self) {
        self.tasks = self.repository.all().to_vec();
    }

    fn sync_to_repo(&mut self) {
        self.repository = TaskRepository::new(self.tasks.clone());
    }

    pub fn generate_task_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    pub fn add_task(&mut self, title: String, description: String) {
        let id = self.generate_task_id();
        let task = Task::new(id, title, description);
        self.repository.add(task.clone());
        self.tasks.push(task);
    }

    pub fn edit_task(&mut self, id: u64, title: String, description: String) {
        if let Some(task) = self.repository.find_by_id_mut(id) {
            task.update_details(title.clone(), description.clone());
        }
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.update_details(title, description);
        }
    }

    pub fn toggle_timer(&mut self, id: u64) {
        if let Some(task) = self.repository.find_by_id_mut(id) {
            task.toggle_timer();
        }
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.toggle_timer();
        }
    }

    pub fn toggle_task(&mut self, id: u64) {
        if let Some(task) = self.repository.find_by_id_mut(id) {
            task.toggle_checked();
        }
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.toggle_checked();
        }
    }

    pub fn clear_all_tasks(&mut self) {
        self.repository.clear();
        self.tasks.clear();
        self.next_id = 0;
    }

    pub fn pause_all_timers(&mut self) {
        for task in self.repository.iter_mut() {
            task.stop_timer();
        }
        for task in &mut self.tasks {
            task.stop_timer();
        }
    }

    pub fn repository(&self) -> &TaskRepository {
        &self.repository
    }

    pub fn repository_mut(&mut self) -> &mut TaskRepository {
        &mut self.repository
    }
}

impl Default for TaskRepository {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

pub struct AppBuilder {
    tasks: Vec<Task>,
    next_id: Option<u64>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: None,
        }
    }

    pub fn with_tasks(mut self, tasks: Vec<Task>) -> Self {
        self.tasks = tasks;
        self
    }

    pub fn with_next_id(mut self, id: u64) -> Self {
        self.next_id = Some(id);
        self
    }

    pub fn add_task(mut self, task: Task) -> Self {
        self.tasks.push(task);
        self
    }

    pub fn build(self) -> App {
        let next_id = self
            .next_id
            .unwrap_or_else(|| self.tasks.iter().map(|t| t.id).max().unwrap_or(0));

        App {
            repository: TaskRepository::new(self.tasks.clone()),
            next_id,
            tasks: self.tasks,
        }
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_repository() {
        let mut repo = TaskRepository::new(vec![]);
        let task = Task::new(1, "Test".into(), "Description".into());
        
        repo.add(task);
        assert_eq!(repo.count(), 1);
        assert_eq!(repo.find_by_id(1).unwrap().title, "Test");
    }

    #[test]
    fn test_app_builder() {
        let app = AppBuilder::new()
            .with_next_id(10)
            .build();

        assert_eq!(app.next_id, 10);
        assert!(app.repository().is_empty());
    }

    #[test]
    fn test_app_operations() {
        let mut app = App::empty();
        
        app.add_task("Task 1".into(), "Description 1".into());
        assert_eq!(app.repository().count(), 1);
        
        app.clear_all_tasks();
        assert!(app.repository().is_empty());
    }

    #[test]
    fn test_repository_filters() {
        let mut repo = TaskRepository::new(vec![]);
        
        let mut task1 = Task::new(1, "Task 1".into(), "Desc".into());
        task1.is_checked = true;
        
        let task2 = Task::new(2, "Task 2".into(), "Desc".into());
        
        repo.add(task1);
        repo.add(task2);
        
        assert_eq!(repo.completed().len(), 1);
        assert_eq!(repo.pending().len(), 1);
    }
}
