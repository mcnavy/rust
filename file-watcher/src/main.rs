use std::sync::mpsc::channel;
use std::path::Path;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    path: String
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            tx.send(res).unwrap();
        },
        notify::Config::default(),
    )?;

    watcher.watch(Path::new(&args.path), RecursiveMode::Recursive)?;

    println!("👀 Watching: {}", args.path);

    for event in rx {
        match event {
            Ok(Event { kind, paths, .. }) => {
                for path in paths {
                    println!("📌 {:?} -> {:?}", kind, path);
                }
            }
            Err(e) => println!("⚠️ Watch error: {:?}", e),
        }
    }

    Ok(())
}
