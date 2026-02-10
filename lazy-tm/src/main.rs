struct App {
    tasks: Vec<Task>,
    next_id: u64,
}

impl App {
    fn new() -> App {
        App {
            tasks: Vec::new(),
            next_id: 0,
        }
    }

    fn generate_task_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    fn add_task(&mut self, title: String, description: String) {
        let id = self.generate_task_id();
        let task = Task::new(id, title, description);

        self.tasks.push(task);
    }

    fn toggle_task(&mut self, id: u64) {
        for task in &mut self.tasks {
            if task.id == id {
                task.is_checked = !task.is_checked;
                break;
            }
        }
    }

    fn remove_task(&mut self, id: u64) {
        for index in 0..self.tasks.len() {
            if self.tasks[index].id == id {
                self.tasks.remove(index);
                break;
            }
        }
    }

    fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

struct Task {
    id: u64,
    title: String,
    description: String,
    is_checked: bool,
}

impl Task {
    fn new(id: u64, title: String, description: String) -> Task {
        Task {
            id,
            title,
            description,
            is_checked: false,
        }
    }
}

fn main() {
    let mut task_list = App::new();

    // Teste de adição de tarefas

    task_list.add_task(String::from("Task 1"), String::from("Description 1"));
    task_list.add_task(String::from("Task 2"), String::from("Description 2"));
    task_list.add_task(String::from("Task 3"), String::from("Description 3"));

    // Teste de listagem das tarefas
    println!("{:-^60}", String::from("Lista de Tasks após adição"));
    for task in task_list.list_tasks() {
        println!("ID: {}", task.id);
        println!("Title: {}", task.title);
        println!("Description: {}", task.description);
        println!("Checked: {}", task.is_checked);
        println!("\n");
    }

    // Teste de alteração de tarefas

    task_list.toggle_task(1);
    task_list.toggle_task(2);

    // Teste de remoção de tarefas

    task_list.remove_task(3);

    println!("{:-^60}", String::from("Lista de Tasks após edições"));
    for task in task_list.list_tasks() {
        println!("ID: {}", task.id);
        println!("Title: {}", task.title);
        println!("Description: {}", task.description);
        println!("Checked: {}", task.is_checked);
        println!("\n");
    }
}
