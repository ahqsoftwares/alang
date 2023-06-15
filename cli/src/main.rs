use std::env::current_dir;

use chalk_rs::Chalk;

use check_update::check_update;
use clap::{crate_version, Command as CliCommand};

use project_create::{dir_empty, err_not_empty, err_reading_dir, get_project_name, get_template, install_template};

use utils::{error, info, launch, show_notes};

static mut VER: Option<String> = None;

mod check_update;
mod project_create;

pub mod check_config;
pub mod utils;

fn main() {
    unsafe {
        VER = Some(format!("v{}", crate_version!()));
    }

    let command = unsafe {
        CliCommand::new("ALang Cli")
            .version(&VER.as_ref().unwrap().as_str())
            .subcommand(
                CliCommand::new("version")
                    .about("Print version")
                    .alias("ver"),
            )
            .subcommand(
                CliCommand::new("releasenotes")
                    .about("Read ReleaseNotes about the latest version")
                    .alias("notes"),
            )
            .subcommand(CliCommand::new("update").about("Update ALang Cli (if available)"))
            .subcommand(CliCommand::new("compile").about("Compile the ALang Project"))
            .subcommand(CliCommand::new("run").about("Compile & Run the ALang Project"))
            .subcommand(
                CliCommand::new("init").about("Initialize an ALang project inside the current dir"),
            )
            .subcommand(
                CliCommand::new("new")
                    .about("Initialize a new ALang project")
                    .alias("create"),
            )
    };

    let mut cloned_cmd = command.clone();

    let matches = command.get_matches();

    let update_available = check_update();

    if let Some(_) = matches.subcommand_matches("version") {
        let data = &cloned_cmd.get_version().unwrap();

        println!("ALang Cli {}", &data);
    } else if let Some(_) = matches.subcommand_matches("init") {
        if let Ok(dir) = current_dir() {
            if let Some(dir) = dir.to_str() {
                if dir_empty(&dir.to_string()) {
                    let template = get_template();
                    install_template(
                        dir.to_string(),
                        template
                    );
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
        let template = get_template();
        let project = get_project_name();

        if let Ok(dir) = current_dir() {
            if let Some(dir) = dir.to_str() {
                if dir_empty(
                    &format!("{}/{}", &dir, &project)
                ) {
                    install_template(
                        format!("{}/{}", &dir, &project),
                        template
                    );
                } else {
                    err_not_empty();
                }
            } else {
                err_reading_dir();
            }
        } else {
            err_reading_dir();
        }
    } else if let Some(_) = matches.subcommand_matches("releasenotes") {
        if let None = show_notes() {
            info("Opening in default browser...");
            let ok = launch("https://github.com/ahqsoftwares/alang/releases/latest");

            if !ok {
                error("Failed to open in default browser\nOpen https://github.com/ahqsoftwares/alang/releases/latest");
            }
        }
    } else if let Some(_) = matches.subcommand_matches("update") {
        info("Coming Soon...");
    } else {
        let _ = &cloned_cmd.print_help();
    }

    if update_available {
        let mut chalk = Chalk::new();

        chalk.blue().bold().println(
            &"A new version of ALang Cli is available\nRun `alang update` to install & `alang releasenotes` to view the release notes"
        );
    }
}
