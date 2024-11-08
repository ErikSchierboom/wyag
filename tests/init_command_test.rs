use std::fs;
use crate::common::binary::Binary;
use crate::common::temp_dir::TempDir;

mod common;

#[test]
fn test_init_in_non_git_dir_succeeds() {
    let binary = Binary::build();
    let temp_dir = TempDir::new();

    let mut command = temp_dir.command(binary.path.to_str().unwrap(), ["init"]);
    let status = command.status().expect("failed to run 'init' command");

    assert!(status.success());
    assert!(fs::exists(temp_dir.path.join(".git")).unwrap());
    assert!(fs::exists(temp_dir.path.join(".git/objects")).unwrap());
    assert!(fs::exists(temp_dir.path.join(".git/refs")).unwrap());
    assert!(fs::exists(temp_dir.path.join(".git/HEAD")).unwrap());
    assert_eq!(fs::read_to_string(temp_dir.path.join(".git/HEAD")).unwrap(), "ref: refs/heads/main\n")
}

#[test]
fn test_init_in_git_dir_fails() {
    let binary = Binary::build();
    let temp_dir = TempDir::new();
    temp_dir.command("git", ["init"]).status().expect("failed to run 'init' command");

    let mut command = temp_dir.command(binary.path.to_str().unwrap(), ["init"]);
    let status = command.status().expect("failed to run 'init' command");

    assert!(!status.success());
    assert!(fs::exists(temp_dir.path.join(".git")).unwrap());
    assert!(fs::exists(temp_dir.path.join(".git/objects")).unwrap());
    assert!(fs::exists(temp_dir.path.join(".git/refs")).unwrap());
    assert!(fs::exists(temp_dir.path.join(".git/HEAD")).unwrap());
}
