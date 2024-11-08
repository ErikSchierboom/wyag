use std::env::temp_dir;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub(crate) struct TempDir {
    pub path: PathBuf,
}

impl Drop for TempDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("could not remove temp dir")
    }
}

impl TempDir {
    pub(crate) fn new() -> Self {
        let path = temp_dir().join(Self::random_dir_name());

        fs::create_dir(&path).expect("error creating temporary directory");

        Self { path }
    }

    pub(crate) fn command<I, S>(&self, program: S, args: I) -> Command
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>,
    {
        let mut cmd = Command::new(program);
        cmd.args(args).current_dir(&self.path);
        cmd
    }

    fn random_dir_name() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }
}