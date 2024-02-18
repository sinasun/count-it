use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

const THREAD_NUM: i8 = 10;

struct File {
    path: PathBuf,
    characters: i32,
    words: i32,
    lines: i32,
}

pub struct Counter {
    undiscorvered_directories: Arc<Mutex<Vec<PathBuf>>>,
    files: Arc<Mutex<Vec<File>>>,
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
                                        shared_files.lock().unwrap().push(File {
                                            path: dir.path(),
                                            characters: 0,
                                            words: 0,
                                            lines: 0,
                                        })
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

    pub fn count_files(&mut self) {
        self.discover_directories();
        let files_list = Arc::clone(&self.files);
        for mut file in &mut files_list.lock().unwrap().drain(..) {
            file.count_file();
            file.print();
        }
    }
}

impl File {
    fn count_file(&mut self) {
        let read_file = fs::read_to_string(self.path.clone());
        match read_file {
            Ok(read_file) => {
                let mut thread_handles = vec![];
                if read_file.is_ascii() {
                    let content = Arc::new(read_file);
                    let content_length = content.len();

                    for i in 0..THREAD_NUM {
                        let content_share = Arc::clone(&content);
                        let content_sub = content_share[i as usize * content_length
                            / THREAD_NUM as usize
                            ..(i as usize + 1) * content_length / THREAD_NUM as usize]
                            .to_owned();
                        thread_handles.push(thread::spawn(move || {
                            let mut characters = 0;
                            let mut words = 0;
                            let mut lines = 0;
                            for ch in content_sub.chars() {
                                if ch.is_ascii() {
                                    let ascii_ch = ch as u32;
                                    // ASCII for space
                                    if ascii_ch == 32 {
                                        words += 1;
                                    } else if ascii_ch == 10 {
                                        // ASCII for new line
                                        words += 1;
                                        lines += 1;
                                    } else if ascii_ch > 32 && ascii_ch < 127 {
                                        characters += 1;
                                    }
                                }
                            }
                            (characters, words, lines)
                        }))
                    }
                }

                for handle in thread_handles {
                    let (sum_char, sum_words, sum_lines) = handle.join().unwrap();
                    self.characters += sum_char;
                    self.words += sum_words;
                    self.lines += sum_lines;
                }
            }
            Err(err) => eprintln!(
                "Error reading file {}: {}",
                self.path.to_string_lossy(),
                err
            ),
        }
    }
    fn print(&self) {
        println!(
            "File: {}, chars: {}, words: {}, lines:{}",
            self.path.to_string_lossy(),
            self.characters,
            self.words,
            self.lines
        );
    }
}
