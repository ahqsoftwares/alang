use std::fs;

use crate::utils::error;

pub fn err_reading_dir() {
    error("> OS Error: Could not access the current directory!");
}

pub fn err_not_empty() {
    error("> The Selected Directory is not empty!");
}

pub fn dir_empty(dir: &String) -> bool {
    return match fs::read_dir(dir) {
        Ok(dir) => {
            return dir.count() == 0
        }
        _ => false
    }
}