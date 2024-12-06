#![forbid(unsafe_code)]

use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf}, sync::atomic::{AtomicUsize, Ordering},
};

use regex::Regex;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Match {
    pub path: PathBuf,
    pub line: String,
    pub line_number: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error {
    pub path: PathBuf,
    pub error: io::Error,
}

pub enum Event {
    Match(Match),
    Error(Error),
}

pub fn run<P: AsRef<Path>>(path: P, pattern: &str, max_depth: i32, current_depth: i32, file_name: &str, counter: &AtomicUsize) -> Vec<Event> {
    let mut subdirs = vec![];
    let mut events = vec![];
    if path.as_ref().is_file() {
        counter.fetch_add(1, Ordering::Relaxed);
        match File::open(path.as_ref()) {
            Ok(mut file) => {
                let re = Regex::new(file_name).expect("wrong regex");
                match path.as_ref().file_name() {
                    Some(file_name) => {
                        if re.is_match(&file_name.to_string_lossy()) {
                            let mut buffer = Vec::new();
                            match file.read_to_end(&mut buffer) {
                                Ok(_) => {
                                    let content = String::from_utf8_lossy(&buffer);
                                    for (i, line) in content.lines().enumerate() {
                                        if line.contains(pattern) {
                                            events.push(Event::Match(Match {
                                                path: path.as_ref().to_path_buf(),
                                                line: line.to_string(),
                                                line_number: i + 1,
                                            }));
                                        }
                                    }
                                },
                                Err(err) => events.push(Event::Error(Error {
                                    path: path.as_ref().to_path_buf(),
                                    error: err,
                                })),
                            }
                        }
                    },
                    None => events.push(Event::Error(Error {
                        path: path.as_ref().to_path_buf(),
                        error: io::Error::new(io::ErrorKind::Other, "bad file name"),
                    })),
                }
            }
            Err(err) => events.push(Event::Error(Error {
                path: path.as_ref().to_path_buf(),
                error: err,
            })),
        }
        return events;
    }
    if current_depth == max_depth {
        return vec![];
    }
    let all_entrys = match path.as_ref().read_dir() {
        Ok(all) => all,
        Err(err) => {
            return vec![Event::Error(Error {
                path: path.as_ref().to_path_buf(),
                error: err,
            })]
        }
    };
    for entry in all_entrys {
        match entry {
            Ok(correct_entry) => {
                subdirs.push(correct_entry.path());
            }
            Err(err) => events.push(Event::Error(Error {
                path: path.as_ref().to_path_buf(),
                error: err,
            })),
        }
    }
    let mut temp_vec = vec![];
    subdirs
        .par_iter()
        .map(|x| run(x, pattern, max_depth, current_depth + 1, file_name, counter))
        .collect_into_vec(&mut temp_vec);
    for i in temp_vec.iter_mut() {
        events.append(i);
    }
    events
}
