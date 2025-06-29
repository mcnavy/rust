use crate::models::Task;

pub fn get_next_id(tasks: &[Task]) -> u32 {
    if tasks.is_empty() {
        1
    } else {
        tasks.iter().map(|x| x.id).max().unwrap() + 1
    }
}

pub fn mark_task_as_completed(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|x| x.id == id) {
        task.completed = true;
    } else {
        println!("Task with ID {} not found", id);
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, id: u32) {
    tasks.retain(|x| x.id != id);
} 