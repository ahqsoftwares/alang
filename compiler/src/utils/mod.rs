mod package;

pub use package::*;

pub fn get_out(dir: &String) -> String {
    format!("{}/out", &dir)
}
