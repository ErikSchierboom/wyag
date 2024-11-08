use std::env::temp_dir;
use std::fs;
use std::path::PathBuf;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub(crate) struct TempDir {
    pub path: PathBuf,
}

impl TempDir {
    pub(crate) fn new() -> Self {
        let path = temp_dir().join(Self::random_name());

        fs::create_dir(&path).expect("error creating temporary directory");

        Self { path }
    }

    fn random_name() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }
}