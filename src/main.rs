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
    Init {
        /// The directory in which to create the Git repository
        #[arg(index = 1, default_value = ".")]
        directory: std::path::PathBuf
    }
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Init { directory}  => commands::init::init_repository(directory)
    }
}
