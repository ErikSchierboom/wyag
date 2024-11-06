use clap::{Parser, Subcommand, ValueEnum};

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
    },
    /// Output a file
    CatFile {
        /// The SHA of the file
        #[arg(index = 1)]
        sha: String
    }
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Init { directory}  => commands::init::init_repository(directory),
        Command::CatFile { sha}  => commands::cat_file::cat(sha),
    }
}
