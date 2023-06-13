use std::env::current_dir;

use check_update::check_update;
use clap::{crate_version, Command as CliCommand};
use project_create::{dir_empty, err_reading_dir, err_not_empty};

static mut VER: Option<String> = None;

mod project_create;
mod check_update;

pub mod check_config;
pub mod utils;

fn main() {
    unsafe {
        VER = Some(format!("v{}", crate_version!()));
    }

    let command = unsafe {
        CliCommand::new("alang")
            .version(&VER.as_ref().unwrap().as_str())
            .subcommand(
                CliCommand::new("version")
                    .about("Print version")
                    .alias("ver"),
            )
            .subcommand(
                CliCommand::new("update")
                    .about("Print version")
                    .alias("ver"),
            )
            .subcommand(
                CliCommand::new("compile")
                    .about("Compile the ALang Project"),
            )
            .subcommand(
                CliCommand::new("run")
                    .about("Compile & Run the ALang Project"),
            )
            .subcommand(
                CliCommand::new("init")
                    .about("Initialize an ALang project inside the current dir"),
            )
            .subcommand(
                CliCommand::new("new")
                    .about("Initialize a new ALang project")
                    .alias("create"),
            )
    };

    let mut cloned_cmd = command.clone();

    let matches = command.get_matches();

    check_update();

    if let Some(_) = matches.subcommand_matches("version") {
        let data = &cloned_cmd.get_version().unwrap();

        println!("ALang Cli v{}", &data);
    } else if let Some(_) = matches.subcommand_matches("init") {
        if let Ok(dir) = current_dir() {
            if let Some(dir) = dir.to_str() {
                if dir_empty(&dir.to_string()) {

                } else {
                    err_not_empty();
                }
            } else {
                err_reading_dir();
            }
        } else {
            err_reading_dir();
        }
    } else if let Some(_) = matches.subcommand_matches("new") {
        
    } else {
        let _ = &cloned_cmd.print_help();
    }
}
