use clap::{Parser, Subcommand};
use kvs::{KvStore, Result, KvError};
use std::env::current_dir;

#[derive(Parser)]
#[command(author=env!("CARGO_PKG_AUTHORS"), version=env!("CARGO_PKG_VERSION"), about=env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Get {
        key1: String,
    },
    Set {
        key1: String,
        value1: String,
    },
    Rm {
        key1: String,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = current_dir()?;
    match &cli.command {
        Some(Commands::Rm { key1 }) => {
            let mut kv = KvStore::open(path)?;
            match kv.remove(key1.to_string()){
                Ok(()) => {},
                Err(KvError::KeyNotFound) => {
                    println!("Key not found");
                    std::process::exit(1);
                },
                Err(e) => {
                    return Err(e);
                }
            };
        }
        Some(Commands::Get { key1 }) => {
            let mut kv = KvStore::open(path)?;
            if let Some(value) = kv.get(key1.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
    
        }
        Some(Commands::Set { key1, value1 }) => {
            let mut kv = KvStore::open(path)?;
            kv.set(key1.to_string(), value1.to_string())?;
        }
        None => {
            unreachable!()
        }
    }
    Ok(())
}
