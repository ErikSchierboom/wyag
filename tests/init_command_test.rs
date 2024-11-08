use std::process::Command;
use crate::common::app::Binary;
use crate::common::temp_dir::TempDir;

mod common;

#[test]
fn test_init_in_non_git_dir_succeeds() {
    let temp_dir = TempDir::new();

    // TODO: create directory without .git directory
    let status = Binary::new(temp_dir.path)
        .run(["init"])
        .expect("failed to run 'init' command");

    assert!(status.success())
}

#[test]
fn test_init_in_git_dir_fails() {
    let temp_dir = TempDir::new();

    Command::new("git")
        .arg("init")
        .current_dir(&temp_dir.path)
        .status()
        .expect("cannot run git init");

    let status = Binary::new(temp_dir.path)
        .run(["init"])
        .expect("failed to run 'init' command");

    assert!(!status.success())
}
