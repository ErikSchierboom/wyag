use std::fs;
use std::path::PathBuf;

const DEFAULT_HEAD: &'static str = "ref: refs/heads/main\n";

pub(crate) fn init_repository(directory: PathBuf) {
    let git_dir = directory.join(".git");
    if git_dir.exists() { panic!("ERROR: the directory already contains a .git directory") }

    fs::create_dir(&git_dir).expect("ERROR: could not create the .git directory");
    fs::create_dir(git_dir.join("objects")).expect("ERROR: could not create the .git/objects directory");
    fs::create_dir(git_dir.join("refs")).expect("ERROR: could not create the .git/refs directory");
    fs::write(git_dir.join("HEAD"), DEFAULT_HEAD).expect("ERROR: could not create .git/HEAD file");
}
