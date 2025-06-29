use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Управление задачами", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Добавить новую задачу в список")]
    Add {
        #[arg(short, long, help = "Название задачи", value_name = "TITLE")] 
        title: String
    }
}

const FILE_PATH: &str = "todo.json";

fn save_tasks(tasks: &[Task]) {
    let json_task = serde_json::to_string(tasks).expect("Failed to serialize tasks");

    std::fs::write(FILE_PATH, json_task).expect("Failed to write tasks to file");
}

fn load_tasks() -> Vec<Task>  {
    let data = std::fs::read_to_string(FILE_PATH).expect("Failed to read tasks from file");
    if data.trim().is_empty() {
        return Vec::new()
    }
    let tasks: Vec<Task> = serde_json::from_str(&data).expect("Failed to deserialize tasks");
    tasks
}

fn get_next_id(tasks: &[Task]) -> u32 {
    if tasks.is_empty() {
        1
    } else {
        tasks.iter().map(|x| x.id).max().unwrap() + 1
    }
}

fn main() {
    let mut tasks = load_tasks();

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => {
            let new_id = get_next_id(&tasks);
            println!("Adding task: {} with ID: {}", title, new_id);

            tasks.push(Task {id: new_id, title, completed: false});
            
            save_tasks(&tasks);

            println!("Task added successfully");
        }
    }
}
