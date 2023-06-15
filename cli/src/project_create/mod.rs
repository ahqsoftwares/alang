use std::{
    fs,
    io::ErrorKind
};

use crate::utils::error;

mod get_name;
mod templates;

pub use get_name::*;
pub use templates::*;

pub fn err_reading_dir() {
    error("> OS Error: Could not access the current directory!");
}

pub fn err_not_empty() {
    error("> The Selected Directory is not empty!");
}

pub fn dir_empty(dir: &String) -> bool {
    return match fs::read_dir(dir) {
        Ok(dir) => return dir.count() == 0,
        Err(error) => {
            match error.kind() {
                ErrorKind::AddrNotAvailable => true,
                ErrorKind::NotFound => true,
                _ => false
            }
            
        },
    };
}
