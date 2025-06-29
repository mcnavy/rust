mod models;
mod cli;
mod storage;
mod task_manager;

use clap::Parser;
use models::Task;
use cli::{Cli, Commands};
use storage::{save_tasks, load_tasks};
use task_manager::{get_next_id, mark_task_as_completed, remove_task};

fn main() {
    let mut tasks = load_tasks();
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => {
            let new_id = get_next_id(&tasks);
            println!("Adding task: {} with ID: {}", title, new_id);

            tasks.push(Task { id: new_id, title, completed: false });
            
            save_tasks(&tasks);

            println!("Task added successfully");
        },
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                for task in tasks {
                    println!("ID: {}, Title: {}, Completed: {}", task.id, task.title, task.completed);
                }
            }
        },
        Commands::Complete { id } => {
            mark_task_as_completed(&mut tasks, id);
            save_tasks(&tasks);
            println!("Task completed successfully");
        },
        Commands::Remove { id } => {
            remove_task(&mut tasks, id);
            save_tasks(&tasks);
            println!("Task removed successfully");
        }
    }
}
