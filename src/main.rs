use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize a new Git repository
    Init
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Init => commands::init::init_repository()
    }
}
