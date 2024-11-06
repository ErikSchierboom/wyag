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
    /// Add file contents to the index
    Add {

    },
    /// Commit file contents from the index
    Commit {

    },
    /// Output a file
    CatFile {
        /// The SHA of the file
        #[arg(index = 1)]
        sha: String
    },
    /// Initialize a new Git repository
    Init {
        /// The directory in which to create the Git repository
        #[arg(index = 1, default_value = ".")]
        directory: std::path::PathBuf
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Add { }  => commands::add::execute(),
        Command::CatFile { sha}  => commands::cat_file::execute(sha),
        Command::Commit { }  => commands::commit::execute(),
        Command::Init { directory}  => commands::init::execute(directory),
    }
}
