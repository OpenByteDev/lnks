use std::{
    env, fs,
    path::{Path, PathBuf},
};

use uuid::Uuid;

pub struct TempPath {
    path: PathBuf,
}

impl TempPath {
    pub fn new(prefix: &str, ext: &str) -> Self {
        let path = env::temp_dir().join(format!(
            "{}-{}-{}.{}",
            prefix,
            env!("CARGO_CRATE_NAME"),
            Uuid::new_v4(),
            ext
        ));

        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempPath {
    fn drop(&mut self) {
        if let Ok(meta) = fs::metadata(&self.path) {
            if meta.is_dir() {
                let _ = fs::remove_dir_all(&self.path);
            } else {
                let _ = fs::remove_file(&self.path);
            }
        }
    }
}
