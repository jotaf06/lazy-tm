mod task;
mod app;

use app::App;

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
