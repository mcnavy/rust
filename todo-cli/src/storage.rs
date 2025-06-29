use crate::models::Task;
use std::fs;

const FILE_PATH: &str = "todo.json";

pub fn save_tasks(tasks: &[Task]) {
    let json_task = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write(FILE_PATH, json_task).expect("Failed to write tasks to file");
}

pub fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string(FILE_PATH).expect("Failed to read tasks from file");
    if data.trim().is_empty() {
        return Vec::new()
    }
    let tasks: Vec<Task> = serde_json::from_str(&data).expect("Failed to deserialize tasks");
    tasks
} 