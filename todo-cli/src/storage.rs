use crate::models::Task;
use std::fs;

pub fn save_tasks(tasks: &[Task]) {
    let json_task = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write(get_data_path(), json_task).expect("Failed to write tasks to file");
}

pub fn load_tasks() -> Vec<Task> {
    let path = get_data_path();

    if !path.exists() {
        return vec![];
    }

    let data = fs::read_to_string(path).expect("Failed to read tasks from file");
    let tasks: Vec<Task> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);
    tasks
} 

fn get_data_path() -> std::path::PathBuf {
    let mut path = dirs::home_dir().expect("Can't find home dir");
    path.push(".todo.json");
    path
}