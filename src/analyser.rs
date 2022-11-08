use std::{cmp::Ordering, fs};

use walkdir::WalkDir;

// Result file
#[derive(Eq)]
pub struct FileResult {
    pub file_dir: String,
    pub file_name: String,
    pub file_size: u64,
}

impl Ord for FileResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_size.cmp(&other.file_size)
    }
}

impl PartialOrd for FileResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FileResult {
    fn eq(&self, other: &Self) -> bool {
        self.file_size == other.file_size
    }
}

pub fn analyse_dir(dir_name: &String) -> Vec<FileResult> {
    let mut files: Vec<FileResult> = Vec::new();

    let dir_md = fs::metadata(dir_name).unwrap_or_else(|error| {
        panic!("Problem analysing directory {}: {:?}", dir_name, error.kind());
    });

    if !dir_md.is_dir() {
        panic!(" {} is not a valid directory", dir_name);
    }

    for entry in WalkDir::new(dir_name)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let meta = fs::metadata(&path).unwrap();

        if !meta.is_dir() {
            let file_size = meta.len();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_dir = path.parent().unwrap().as_os_str().to_str().unwrap();
            let result = FileResult {
                file_dir: String::from(file_dir),
                file_name: String::from(file_name),
                file_size: file_size,
            };
            files.push(result);
        }
    }
    files.sort_by(|a, b| a.cmp(b));
    files.reverse();

    return files;
}