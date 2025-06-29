use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Управление задачами", version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Добавить новую задачу в список")]
    Add {
        #[arg(short, long, help = "Название задачи", value_name = "TITLE")] 
        title: String
    },
    #[command(about = "Показать все задачи")]
    List,
    #[command(about = "Отметить задачу как выполненную")]
    Complete {
        #[arg(short, long, help = "ID задачи", value_name = "ID")]
        id: u32
    },
    #[command(about = "Удалить задачу")]
    Remove {
        #[arg(short, long, help = "ID задачи", value_name = "ID")]
        id: u32
    }
} 