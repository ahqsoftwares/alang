use std::{
    fs,
    io::{stdin, ErrorKind, Read},
    process,
};

use chalk_rs::Chalk;
use inquire::{Confirm, Text};

mod download;
mod get_urls;
use download::*;
use get_urls::*;

fn main() {
    let ver = env!("CARGO_PKG_VERSION");
    #[cfg(windows)]
    let os = "Windows";

    #[cfg(target_os = "linux")]
    let os = "Linux";

    #[cfg(target_os = "macos")]
    let os = "Macos";

    success(format!("ALang {} Installer v{}", &os, ver).as_str());

    let text = Text::new("Where should we install alang?").with_default((|| {
        if cfg!(windows) {
            "C:\\alang"
        } else if cfg!(target_os = "linux") {
            "/alang"
        } else {
            "/alang"
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
            if let Ok(dir) = fs::read_dir(&data) {
                if dir.count() != 0 {
                    err("The selected directory contains files");
                }
            }

            if let Err(_) = fs::create_dir_all(format!("{}/downloads", &data)) {
                err(format!("Could not make directory at {}", &data).as_str());
            }

            info("> Getting latest release...");

            let alang_release = get_urls();

            info(format!("> Installing v{}", &alang_release.version).as_str());

            info("> Downloading ALang cli for all platforms..");

            alang_release
                .assets
                .iter()
                .for_each(|asset| match &asset.class {
                    AssetClass::WindowsCli => {
                        info("  > Downloading ALang Cli for Windows");
                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "cli_windows.exe".to_owned(),
                            |current, total| {
                                println!(
                                    "      > {}% downloaded (Windows Cli)",
                                    (current * 100) / total
                                );
                            },
                        );
                    }
                    AssetClass::MacosCli => {
                        info("  > Downloading ALang Cli for Macos");
                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "cli_macos".to_owned(),
                            |current, total| {
                                println!(
                                    "      > {}% downloaded (Macos Cli)",
                                    (current * 100) / total
                                );
                            },
                        );
                    }
                    AssetClass::LinuxCli => {
                        info("  > Downloading ALang Cli for Linux");
                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "cli_linux".to_owned(),
                            |current, total| {
                                println!(
                                    "      > {}% downloaded (Linux Cli)",
                                    (current * 100) / total
                                );
                            },
                        );
                    }
                    _ => {}
                });

            success("> ALang Cli Downloaded");

            info("> Downloading tools for all platforms...");

            alang_release
                .assets
                .iter()
                .for_each(|asset| match &asset.class {
                    AssetClass::WindowsTools => {
                        info("  > Downloading ALang Tools for Windows");
                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "tools_windows.zip".to_owned(),
                            |current, total| {
                                println!(
                                    "      > {}% downloaded (Windows)",
                                    (current * 100) / total
                                );
                            },
                        );
                    }
                    AssetClass::MacosTools => {
                        info("  > Downloading ALang Tools for Macos");
                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "tools_macos.zip".to_owned(),
                            |current, total| {
                                println!("      > {}% downloaded (Macos)", (current * 100) / total);
                            },
                        );
                    }
                    AssetClass::LinuxTools => {
                        info("  > Downloading ALang Tools for Linux");
                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "tools_linux.zip".to_owned(),
                            |current, total| {
                                println!("      > {}% downloaded (Linux)", (current * 100) / total);
                            },
                        );
                    }
                    _ => {}
                });

            success("> ALang Tools downloaded");

            info("> Installing..");

            if let Err(_) = fs::create_dir_all(format!("{}/tools", &data)) {
                err("Could not create tools folder");
            }

            #[cfg(windows)]
            let _to_extract = format!("{}/downloads/tools_windows.zip", &data);

            #[cfg(target_os = "macos")]
            let _to_extract = format!("{}/downloads/tools_macos.zip", &data);

            #[cfg(target_os = "linux")]
            let _to_extract = format!("{}/downloads/tools_linux.zip", &data);

            success("> ALang has been installed! 🎉");

            println!("-------------------------------------------------------");

            println!(
                r#"░█████╗░██╗░░░░░░█████╗░███╗░░██╗░██████╗░
██╔══██╗██║░░░░░██╔══██╗████╗░██║██╔════╝░
███████║██║░░░░░███████║██╔██╗██║██║░░██╗░
██╔══██║██║░░░░░██╔══██║██║╚████║██║░░╚██╗
██║░░██║███████╗██║░░██║██║░╚███║╚██████╔╝
╚═╝░░╚═╝╚══════╝╚═╝░░╚═╝╚═╝░░╚══╝░╚═════╝░"#
            );

            println!("-------------------------------------------------------");

            print!("> ALang Path: ");

            let mut chalk = Chalk::new();

            chalk.yellow().bold().print(&data);

            print!(" <\n> ");
            chalk
                .green()
                .bold()
                .print(&"Add the above install dir to your path variable");
            println!(" <");

            println!("-------------------------------------------------------");

            info("Press enter key to exit");

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
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return true;
            } else {
                return false;
            }
        }
        Err(err) => {
            let err = err.kind();

            match err {
                ErrorKind::NotFound => return false,
                _ => return true,
            }
        }
    }
}

fn info(data: &str) {
    let mut chalk = Chalk::new();

    chalk.yellow().bold().println(&data);
}

fn err(data: &str) {
    let mut chalk = Chalk::new();

    chalk.red().bold().println(&data);

    info("Press enter key to exit");

    let mut buf = [0; 1];
    let _ = stdin().read_exact(&mut buf);

    process::exit(1);
}

fn success(data: &str) {
    let mut chalk = Chalk::new();

    chalk.green().bold().println(&data);
}
