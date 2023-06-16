mod compile_code;
mod read_packages;

use std::env::current_dir;

use read_packages::get_package;

use crate::{info, success};

pub fn compile(dir: String) {
    let dir = (|| {
        let dir = dir.clone();
        let dir = dir.as_str();

        if dir == "." {
            current_dir().unwrap().to_str().unwrap().to_string()
        } else {
            dir.to_string()
        }
    })();

    info("Compiling Package");

    let _ = get_package(&dir);

    success("Compiled Package");

    let _ = compile_code::compile(&dir);
}
