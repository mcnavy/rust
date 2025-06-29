mod models;
mod cli;
mod storage;
mod task_manager;

use clap::Parser;
use colored::*;
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

            tasks.push(Task { id: new_id, title, completed: false });
            
            save_tasks(&tasks);

            println!("{}", "Task added successfully".green());
        },
        Commands::List => {
            if tasks.is_empty() {
                println!("{}", "Task not found".red());
            } else {
                for task in tasks {
                    if task.completed {
                        println!("✅ {} {}", task.id, task.title.green());
                    } else {
                        println!("⏳ {} {}", task.id, task.title.white());
                    }
                }
            }
        },
        Commands::Complete { id } => {
            mark_task_as_completed(&mut tasks, id);
            save_tasks(&tasks);
            println!("{}", "Task completed successfully".blue());
        },
        Commands::Remove { id } => {
            remove_task(&mut tasks, id);
            save_tasks(&tasks);
            println!("{}", "Task removed successfully".yellow());
        }
    }
}
