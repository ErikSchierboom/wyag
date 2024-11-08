use std::{env, path};
use std::process::Command; // Run programs

fn target_dir() -> path::PathBuf {
    env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
        .expect("this should only be used where a `current_exe` can be set")
}

#[test]
fn test_add() {
    Command::new("cargo").arg("build").status().expect("Cannot build");
    let binary = target_dir().join(format!("{}{}", env!("CARGO_PKG_NAME"), env::consts::EXE_SUFFIX));


    // todo!("qweeq")

    let mut cmd = Command::new(binary).arg("init").status().expect("Should succeed");
}
