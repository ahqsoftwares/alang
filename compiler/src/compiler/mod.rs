mod compile_code;
mod read_packages;

use std::env::current_dir;

use read_packages::get_package;

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

    let _ = get_package(&dir);
    let _ = compile_code::compile(&dir);
}
