/*
Primary purpose is to provide commonly used utility functions for file operations.
 */

// compare two directories and return true if they are the same
// compare both the structure and the contents of the files
// compare contents of files by hashing them
use std::collections::HashSet;
use std::fs;
// Import the log crate
use log::{trace};

fn get_path_split_index(filename: &str, unwrap_depth: usize) -> usize {
    let split_char = '/';
    let split_idx = filename.char_indices()
        .filter(|&(_, c)| c == split_char)
        .nth(unwrap_depth - 1)
        .map(|(i, _)| i)
        .unwrap_or(filename.len());
    split_idx
}

fn strip_outer_directories(files: HashSet<String>, strip_depth: usize) -> HashSet<String> {
    // strip the outer directories from the files
    // based on the strip depth
    // return the stripped files
    let mut stripped_files = HashSet::new();
    for file in files {
        let mut stripped_file = file.clone();
        trace!("file before strip: {}", stripped_file);
        let split_idx = get_path_split_index(&stripped_file, strip_depth);
        stripped_file = stripped_file.split_at(split_idx).1.to_string();
        trace!("file after strip: {}", stripped_file);
        stripped_files.insert(stripped_file);
    }
    stripped_files
}

fn get_files_in_directory(dir: &str) -> HashSet<String> {
    // get all files in directory
    // return set of files
    let mut files = HashSet::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        files.insert(path_str.to_string());
    }
    files
}

pub fn compare_directories(dir1: &str, dir2: &str, strip_depth: usize) -> bool {
    // initial approach, 
    // 1. get set of files in dir1
    // 2. get set of files in dir2
    // 3. compare sets
    // 4. if sets are equal, compare contents of files
    // 5. if contents are equal, return true
    // 6. else return false
    let mut dir1_files = get_files_in_directory(dir1);
    let mut dir2_files = get_files_in_directory(dir2);
    // we need to strip the outer directories, based on the strip depth
    dir1_files = strip_outer_directories(dir1_files, strip_depth);
    dir2_files = strip_outer_directories(dir2_files, strip_depth);
    // now, compare and return result
    trace!("dir1_files: {:?}", dir1_files);
    trace!("dir2_files: {:?}", dir2_files);
    dir1_files == dir2_files
}