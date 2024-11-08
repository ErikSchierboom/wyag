use std::fs;
use std::io::Read;
use crate::common::binary::Binary;
use crate::common::temp_dir::TempDir;

mod common;

#[test]
fn test_cat_file_with_known_sha_succeeds() {
    let binary = Binary::build();
    let temp_dir = TempDir::new();

    fs::write(temp_dir.path.join("test.txt"), "hello world\n").expect("failed to write test file");
    temp_dir.command("git", ["init"]).status().expect("failed to run 'git init' command");
    temp_dir.command("git", ["add", "test.txt"]).status().expect("failed to run 'git add' command");
    temp_dir.command("git", ["commit", "-m", "initial commit"]).spawn().expect("failed to run 'git commit' command");

    let mut command = temp_dir.command(binary.path.to_str().unwrap(), ["cat-file", "3b18e512dba79e4c8300dd08aeb37f8e728b8dad"]);
    let output = command.output().expect("whoops");
    let std = String::from_utf8(output.stdout).expect("whoops2");
    assert_eq!(std, "blob 12\0hello world\n\n");
}

#[test]
fn test_cat_file_with_unknown_sha_fails() {
    let binary = Binary::build();
    let temp_dir = TempDir::new();

    fs::write(temp_dir.path.join("test.txt"), "hello world\n").expect("failed to write test file");
    temp_dir.command("git", ["init"]).status().expect("failed to run 'git init' command");

    let mut command = temp_dir.command(binary.path.to_str().unwrap(), ["cat-file", "3b18e512dba79e4c8300dd08aeb37f8e728b8dad"]);
    let status = command.status().expect("whoops");
    assert!(!status.success());
}
