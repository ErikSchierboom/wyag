use crate::common::{app, AppBinary};

mod common;

#[test]
fn test_init_in_non_git_dir_succeeds() {
    // TODO: create directory without .git directory
    let status = AppBinary::new()
        .run(["init"])
        .expect("failed to run 'init' command");

    assert!(status.success())
}

#[test]
fn test_init_in_git_dir_fails() {
    // TODO: create directory with .git directory
    let status = app::Binary::new()
        .run(["init"])
        .expect("failed to run 'init' command");

    assert!(!status.success())
}
