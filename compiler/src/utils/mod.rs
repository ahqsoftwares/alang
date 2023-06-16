mod package;

use std::fmt::Display;

pub use package::*;

use chalk_rs::Chalk;

pub fn get_out(dir: &String) -> String {
    format!("{}/out", &dir)
}

pub fn info<T: Display>(data: T) {
    let mut chalk = Chalk::new();

    chalk.blue().bold().print(&"INFO    ");
    println!("{}", data);
}

pub fn warn<T: Display>(data: T) {
    let mut chalk = Chalk::new();

    chalk.yellow().bold().print(&"WARN    ");
    chalk.println(&data);
}

pub fn err<T: Display>(data: T) {
    let mut chalk = Chalk::new();

    chalk.red().bold().print(&"ERROR   ");
    chalk.println(&data);
}

pub fn success<T: Display>(data: T) {
    let mut chalk = Chalk::new();

    chalk.green().bold().print(&"SUCCESS ");
    println!("{}", data);
}