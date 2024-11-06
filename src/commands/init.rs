use std::{fs, env};

// TODO: replace expect/panic! with returning custom error

pub(crate) fn init_repository() {
    let working_dir = env::current_dir().expect("ERROR: could not get current directory");
    let git_dir = working_dir.join(".git");
    if git_dir.exists() { panic!("ERROR: the .git directory already exists") }

    fs::create_dir(git_dir).expect("ERROR: could not create .git directory")
}
