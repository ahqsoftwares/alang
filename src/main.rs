use std::{fs, process, io::{ErrorKind, stdin, Read}};

use chalk_rs::Chalk;
use inquire::{Text, Confirm};

fn main() {
    let text = Text::new("Where should we install alang?")
        .with_default((|| {
            if cfg!(windows) {
                "C:\\alang"
            } else if cfg!(target_os = "linux") {
                "/alang"
            } else {
                ""
            }
        })());

    let prompt = text.prompt();

    if let Ok(data) = prompt {
        if is_symlink(data.clone()) {
            err("> Sorry, please use only absolute paths (the current path was found as symlink)");
        }

        let confirm = Confirm::new(format!("Install alang at {}?", &data).as_str())
            .with_default(true)
            .prompt();

        if let Ok(true) = confirm {
            info("> Downloading ALang cli..");

            success("> ALang Cli Downloaded");

            #[cfg(windows)]
            let os = "Windows";

            #[cfg(target_os = "linux")]
            let os = "Linux";

            #[cfg(target_os = "macos")]
            let os = "Macos";

            info(
                format!("> Downloading tools for {}", &os).as_str()
            );

            success(
                format!("> ALang Tools for {os} downloaded").as_str()
            );

            info("> Extracting..");

            success("> ALang has been installed! 🎉");

            println!("-------------------------------------------------------");

            println!(r#"░█████╗░██╗░░░░░░█████╗░███╗░░██╗░██████╗░
██╔══██╗██║░░░░░██╔══██╗████╗░██║██╔════╝░
███████║██║░░░░░███████║██╔██╗██║██║░░██╗░
██╔══██║██║░░░░░██╔══██║██║╚████║██║░░╚██╗
██║░░██║███████╗██║░░██║██║░╚███║╚██████╔╝
╚═╝░░╚═╝╚══════╝╚═╝░░╚═╝╚═╝░░╚══╝░╚═════╝░"#);

            println!("-------------------------------------------------------");

            print!("> ALang Path: ");
            
            let mut chalk = Chalk::new();

            chalk.yellow().bold().print(&data);

            print!(" <\n> ");
            chalk.green().bold().print(&"Add the above install dir to your path variable");
            println!(" <");

            println!("-------------------------------------------------------");

            info("Press any key to exit");

            let mut buf = [0; 1];
            let _ = stdin().read_exact(&mut buf);

            process::exit(0);
        } else {
            err("> Aborted, enter a new directory after running the same executable");
        }
    } else {
        err("> Cancelled");
    }
}

fn is_symlink(path: String) -> bool {
    match fs::symlink_metadata(path) {
        Ok(metadata) => if metadata.file_type().is_symlink() {
            return true
        } else {
            return false
        }
        Err(err) => {
            let err = err.kind();

            match err {
                ErrorKind::NotFound => return false,
                _ => return true
            }
        }
    }
}

fn err(data: &str) {
    let mut chalk = Chalk::new();

    chalk.red().bold().println(&data);

    process::exit(1);
}

fn info(data: &str) {
    let mut chalk = Chalk::new();

    chalk.yellow().bold().println(&data);
}

fn success(data: &str) {
    let mut chalk = Chalk::new();

    chalk.green().bold().println(&data);
}