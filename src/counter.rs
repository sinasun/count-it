use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Counter {
    undiscorvered_directories: Arc<Mutex<Vec<PathBuf>>>,
    files: Arc<Mutex<Vec<PathBuf>>>,
    thread_handles: Vec<thread::JoinHandle<()>>,
}

impl Counter {
    pub fn build(path: &String) -> Result<Counter, Error> {
        let absolute_path = fs::canonicalize(path);
        match absolute_path {
            Ok(path) => {
                return Ok(Counter {
                    undiscorvered_directories: Arc::new(Mutex::new(vec![path])),
                    files: Arc::new(Mutex::new(vec![])),
                    thread_handles: vec![],
                });
            }
            Err(error) => return Err(error),
        }
    }

    pub fn discover_directories(&mut self) {
        loop {
            let shared_directories = Arc::clone(&self.undiscorvered_directories);
            let shared_files = Arc::clone(&self.files);
            if let Some(directory) = self.get_next_directory() {
                self.thread_handles.push(thread::spawn(move || {
                    if let Ok(entries) = fs::read_dir(directory) {
                        for entry in entries {
                            if let Ok(dir) = entry {
                                if let Ok(file_type) = dir.file_type() {
                                    let path = dir.path();
                                    if file_type.is_dir() {
                                        shared_directories.lock().unwrap().push(path);
                                    } else if file_type.is_file() {
                                        shared_files.lock().unwrap().push(dir.path());
                                    }
                                }
                            }
                        }
                    }
                }));
            } else {
                let thread_count = Arc::strong_count(&shared_directories);

                if thread_count < 3 {
                    break;
                }
            }
        }
        for handle in self.thread_handles.drain(..) {
            if let Err(e) = handle.join() {
                eprintln!("Error joining thread: {:?}", e);
            }
        }
    }

    fn get_next_directory(&mut self) -> Option<PathBuf> {
        let mut directories = self.undiscorvered_directories.lock().unwrap();
        if directories.is_empty() {
            return None;
        }
        Some(directories.remove(0))
    }
}
