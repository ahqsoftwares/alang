use serde::{Deserialize, Serialize};
#[cfg(not(debug_assertions))]
use std::env::current_exe;
use std::process;

use open::that;

use chalk_rs::Chalk;

mod get_release_notes;

pub use get_release_notes::*;

#[cfg(not(debug_assertions))]
pub fn get_install_dir() -> Option<String> {
    if let Ok(exec) = current_exe() {
        if let Some(dir) = exec.to_str() {
            let exec = dir.to_string();

            let mut splits = exec
                .split((|| {
                    if cfg!(windows) {
                        "\\"
                    } else {
                        "/"
                    }
                })())
                .map(|e| e.clone().to_string())
                .collect::<Vec<String>>();

            splits.pop();

            Some(splits.join((|| {
                if cfg!(windows) {
                    "\\"
                } else {
                    "/"
                }
            })()))
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(debug_assertions)]
pub fn get_install_dir() -> Option<String> {
    let exec = "E:\\alang\\alang.exe".to_owned();

    let mut splits = exec
        .split((|| {
            if cfg!(windows) {
                "\\"
            } else {
                "/"
            }
        })())
        .map(|e| e.clone().to_string())
        .collect::<Vec<String>>();

    splits.pop();

    Some(splits.join((|| {
        if cfg!(windows) {
            "\\"
        } else {
            "/"
        }
    })()))
}

pub fn error(report: &str) {
    let mut chalk = Chalk::new();

    chalk.red().bold().println(&report);
    process::exit(1);
}

pub fn info(report: &str) {
    let mut chalk = Chalk::new();

    chalk.yellow().bold().println(&report);
}

pub fn launch(url: &str) -> bool {
    match that(url) {
        Ok(_) => true,
        _ => false,
    }
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ALangRelease {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}
