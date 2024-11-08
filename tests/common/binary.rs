use std::env;
use std::path::PathBuf;
use std::process::Command;

pub(crate) struct Binary {
    pub path: PathBuf
}

impl Binary {
    pub(crate) fn build() -> Self {
        // Ensure we're testing the latest version of the code
        Command::new("cargo")
            .arg("build")
            .status()
            .expect("cannot build binary");

        Self { path: Self::path() }
    }

    fn path() -> PathBuf {
        Self::dir().join(Self::filename())
    }

    fn dir() -> PathBuf {
        env::current_exe().ok()
            .map(|mut path| {
                path.pop();
                if path.ends_with("deps") {
                    path.pop();
                }
                path
            })
            .expect("this should only be used where a `current_exe` can be set")
    }

    fn filename() -> String {
        format!("{}{}", env!("CARGO_PKG_NAME"), env::consts::EXE_SUFFIX)
    }
}
