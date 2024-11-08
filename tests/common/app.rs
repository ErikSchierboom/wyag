use std::{env, path};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

pub(crate) struct Binary {
    path: PathBuf,
    dir: PathBuf
}

impl Binary {
    pub(crate) fn new(dir: PathBuf) -> Self {
        Self::build();

        Self { path: Self::path(), dir }
    }

    pub(crate) fn run<I, S>(&self, args: I) -> std::io::Result<ExitStatus>
        where
            I: IntoIterator<Item = S>,
            S: AsRef<OsStr>
    {
        Command::new(&self.path).args(args).current_dir(&self.dir).status()
    }

    fn build() {
        Command::new("cargo").arg("build").status().expect("cannot build binary");
    }

    fn path() -> PathBuf {
        let dir = env::current_exe().ok()
            .map(|mut path| {
                path.pop();
                if path.ends_with("deps") {
                    path.pop();
                }
                path
            })
            .expect("this should only be used where a `current_exe` can be set");
        let file = format!("{}{}", env!("CARGO_PKG_NAME"), env::consts::EXE_SUFFIX);
        dir.join(file)
    }
}
