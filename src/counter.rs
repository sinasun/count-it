use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct Counter {
    undiscorvered_directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

impl Counter {
    pub fn build(path: &String) -> Result<Counter, Error> {
        let absolute_path = fs::canonicalize(path);
        match absolute_path {
            Ok(path) => {
                return Ok(Counter {
                    undiscorvered_directories: vec![path],
                    files: Vec::new(),
                });
            }
            Err(error) => return Err(error),
        }
    }

    pub fn discover_directories(&self) {
        dbg!(&self.undiscorvered_directories);
    }
}
