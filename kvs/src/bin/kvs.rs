use clap::{Parser, Subcommand};
use std::process::exit;
use kvs::Result;

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
fn main() -> Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Rm { key1 }) => {
            eprintln!("unimplemented");
            exit(1)
        }
        Some(Commands::Get { key1 }) => {
            eprintln!("unimplemented");
            exit(1)
        }
        Some(Commands::Set { key1, value1 }) => {
            eprintln!("unimplemented");
            exit(1)
        }
        None => {
            println!("no op applied");
            unreachable!()
        }
    }
}
