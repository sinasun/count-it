use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct Counter {
    undiscorvered_directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

impl Counter {
    pub fn new(path: &String) -> Result<Counter, Error> {
        let absolute_path = match fs::canonicalize(path) {
            Ok(path) => path,
            Err(_err) => {
                panic!("Cannot find the file/directory");
            }
        };

        return Ok(Counter {
            undiscorvered_directories: vec![absolute_path],
            files: Vec::new(),
        });
    }

    pub fn discover_directories(&self) {
        dbg!(&self.undiscorvered_directories);
    }
}
