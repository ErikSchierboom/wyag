use std::fmt::{Display, Formatter};
use std::path::PathBuf;
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
    /// Hash an object
    HashObject {
        /// Write the hash into the repository
        #[arg(short, default_value = "false")]
        write: bool,

        #[arg(short = 't', default_value = "blob")]
        object_type: ObjectType,

        /// The file which contents to hash
        #[arg(index = 1)]
        file: PathBuf
    },
    /// Initialize a new Git repository
    Init {
        /// The directory in which to create the Git repository
        #[arg(index = 1, default_value = ".")]
        directory: PathBuf
    },
}

/// Specify the type of object
#[derive(ValueEnum, Debug, Clone)]
enum ObjectType {
    Commit,
    Tree,
    Blob,
    Tag
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Commit => f.write_str("commit"),
            Self::Tree => f.write_str("tree"),
            Self::Blob => f.write_str("blob"),
            Self::Tag => f.write_str("tag"),
        }
    }
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Add { }  => commands::add::execute(),
        Command::CatFile { sha}  => commands::cat_file::execute(sha),
        Command::Commit { }  => commands::commit::execute(),
        Command::HashObject { write, file, object_type} => commands::hash_object::execute(write, file, object_type),
        Command::Init { directory}  => commands::init::execute(directory),
    }
}
