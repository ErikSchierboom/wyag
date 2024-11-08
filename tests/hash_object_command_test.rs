use std::fs;
use std::io::Read;
use crate::common::binary::Binary;
use crate::common::temp_dir::TempDir;

mod common;

#[test]
fn test_hash_object_without_write_flag_and_existing_file_succeeds() {
    let binary = Binary::build();
    let temp_dir = TempDir::new();

    fs::write(temp_dir.path.join("test.txt"), "hello world").expect("failed to write test file");
    temp_dir.command("git", ["init"]).status().expect("failed to run 'git init' command");

    let mut command = temp_dir.command(binary.path.to_str().unwrap(), ["hash-object", "test.txt"]);
    let output = command.output().expect("whoops");
    let std = String::from_utf8(output.stdout).expect("whoops2");
    assert_eq!(std, "95d09f2b10159347eece71399a7e2e907ea3df4f\n");
}

#[test]
fn test_hash_object_without_write_flag_and_non_existing_file_fails() {
    let binary = Binary::build();
    let temp_dir = TempDir::new();

    temp_dir.command("git", ["init"]).status().expect("failed to run 'git init' command");

    let mut command = temp_dir.command(binary.path.to_str().unwrap(), ["hash-object", "missing.txt"]);
    let status = command.status().expect("whoops");
    assert!(!status.success());
}

#[test]
fn test_hash_object_with_write_flag_outputs_object() {
    let binary = Binary::build();
    let temp_dir = TempDir::new();

    fs::write(temp_dir.path.join("test.txt"), "hello world").expect("failed to write test file");
    temp_dir.command("git", ["init"]).status().expect("failed to run 'git init' command");

    let mut command = temp_dir.command(binary.path.to_str().unwrap(), ["hash-object", "-w", "test.txt"]);
    assert!(command.status().expect("failed to get status").success());
    assert!(fs::exists(temp_dir.path.join(".git/objects/95/d09f2b10159347eece71399a7e2e907ea3df4f")).expect("whoops"))
}
