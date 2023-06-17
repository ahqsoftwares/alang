use std::{
    fs,
    io::{stdin, ErrorKind, Read},
    panic::catch_unwind,
    process,
    time::{SystemTime, UNIX_EPOCH, Duration}, env, thread,
};

use chalk_rs::Chalk;
use indicatif::{ProgressBar, ProgressDrawTarget};
use inquire::{Confirm, Text};

mod download;
mod get_urls;
#[cfg(target_os = "linux")]
mod permission;
use download::*;
use get_urls::*;

#[cfg(target_os = "linux")]
use permission::*;

use zip::read::ZipArchive;

static mut PROGRESSBAR: Option<ProgressBar> = None;

fn main() {
    let mut i = 0;
    let args = env::args().collect::<Vec<String>>().iter().filter(|_| {
        i += 1;
        i != 1
    }).map(|x|x.to_string()).collect::<Vec<String>>();

    if !args.is_empty() {
        println!("{:#?}", args);
    }

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

    let prompt = (|| {
        if args.len() == 0 || args[0].as_str() != "-f" {
            text.prompt()
        } else {
            info("> Treating the file path as updater");
            info("> Will Wipe the Directory in case it exists & contains files");
            info("> If symlinks are used it will remove the symlink itself");
            red("> Please run Ctrl+C if alang cli didn't ran this command.", false);
            red("> You have 10secs", false);
            
            thread::sleep(Duration::from_secs(3));
            
            for i in 0..10 {
                info(format!("> Preparing to install in {}secs\nUse Ctrl+C if `alang update` command didn't ran it\n", 10 - i).as_str());
                thread::sleep(Duration::from_secs(1));
            }
            Ok(args[1].to_string())
        }
    })();

    if let Ok(data) = prompt {
        if is_symlink(data.clone()) {
            err("> Sorry, please use only absolute paths (the current path was found as symlink)");
        }

        let confirm = (|| {
            if args.len() == 0 || args[0].as_str() != "-f" {
                Confirm::new(format!("Install alang at {}?", &data).as_str())
                    .with_default(true)
                    .prompt()
            } else {
                Ok(true)
            }
        })();

        if let Ok(true) = confirm {
            if let Ok(dir) = fs::read_dir(&data) {
                if dir.count() != 0 {
                    if args.len() == 0 || args[0].as_str() != "-f" {
                        err("The selected directory contains files");
                    } else {
                        red("The dir contains files; Foce Removing the dir", false);
                        fs::remove_dir_all(&data).unwrap();
                    }
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
                        let bar: ProgressBar = ProgressBar::new(100);
                        bar.set_draw_target(ProgressDrawTarget::stdout());

                        unsafe {
                            PROGRESSBAR = Some(bar);
                        }

                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "cli_windows.exe".to_owned(),
                            |c, t| unsafe {
                                let perc = (c * 100) / t;

                                PROGRESSBAR.as_ref().unwrap().set_position(perc);
                            },
                        );

                        unsafe {
                            PROGRESSBAR.as_ref().unwrap().finish();
                        }
                    }
                    AssetClass::MacosCli => {
                        info("  > Downloading ALang Cli for Macos");
                        let bar = ProgressBar::new(100);
                        bar.set_draw_target(ProgressDrawTarget::stdout());

                        unsafe {
                            PROGRESSBAR = Some(bar);
                        }

                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "cli_macos".to_owned(),
                            |current, total| unsafe {
                                let perc = (current * 100) / total;

                                PROGRESSBAR.as_ref().unwrap().set_position(perc);
                            },
                        );

                        unsafe {
                            PROGRESSBAR.as_ref().unwrap().finish();
                        }
                    }
                    AssetClass::LinuxCli => {
                        info("  > Downloading ALang Cli for Linux");
                        let bar = ProgressBar::new(100);
                        bar.set_draw_target(ProgressDrawTarget::stdout());

                        unsafe {
                            PROGRESSBAR = Some(bar);
                        }

                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "cli_linux".to_owned(),
                            |current, total| unsafe {
                                let perc = (current * 100) / total;

                                PROGRESSBAR.as_ref().unwrap().set_position(perc);
                            },
                        );

                        unsafe {
                            PROGRESSBAR.as_ref().unwrap().finish();
                        }
                    }
                    _ => {}
                });

            success("> ALang Cli Downloaded");

            info("> Downloading tools for all platforms...");

            alang_release
                .assets
                .iter()
                .for_each(|asset| match &asset.class {
                    AssetClass::CodeTemplates => {
                        info("  > Downloading ALang Code Templates");
                        let bar = ProgressBar::new(100);
                        bar.set_draw_target(ProgressDrawTarget::stdout());

                        unsafe {
                            PROGRESSBAR = Some(bar);
                        }

                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "templates.zip".to_owned(),
                            |current, total| unsafe {
                                let progress = (current * 100) / total;
                                PROGRESSBAR.as_ref().unwrap().set_position(progress);
                            },
                        );

                        unsafe {
                            PROGRESSBAR.as_ref().unwrap().finish();
                        }
                    }
                    AssetClass::WindowsTools => {
                        info("  > Downloading ALang Tools for Windows");

                        let bar = ProgressBar::new(100);
                        bar.set_draw_target(ProgressDrawTarget::stdout());

                        unsafe {
                            PROGRESSBAR = Some(bar);
                        }

                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "tools_windows.zip".to_owned(),
                            |current, total| unsafe {
                                let perc = (current * 100) / total;

                                PROGRESSBAR.as_ref().unwrap().set_position(perc);
                            },
                        );

                        unsafe {
                            PROGRESSBAR.as_ref().unwrap().finish();
                        }
                    }
                    AssetClass::MacosTools => {
                        info("  > Downloading ALang Tools for Macos");

                        let bar = ProgressBar::new(100);
                        bar.set_draw_target(ProgressDrawTarget::stdout());

                        unsafe {
                            PROGRESSBAR = Some(bar);
                        }

                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "tools_macos.zip".to_owned(),
                            |current, total| unsafe {
                                let perc = (current * 100) / total;

                                PROGRESSBAR.as_ref().unwrap().set_position(perc);
                            },
                        );

                        unsafe {
                            PROGRESSBAR.as_ref().unwrap().finish();
                        }
                    }
                    AssetClass::LinuxTools => {
                        info("  > Downloading ALang Tools for Linux");

                        let bar = ProgressBar::new(100);
                        bar.set_draw_target(ProgressDrawTarget::stdout());

                        unsafe {
                            PROGRESSBAR = Some(bar);
                        }

                        download(
                            asset.url.clone(),
                            format!("{}/downloads", &data),
                            "tools_linux.zip".to_owned(),
                            |current, total| unsafe {
                                let perc = (current * 100) / total;

                                PROGRESSBAR.as_ref().unwrap().set_position(perc);
                            },
                        );

                        unsafe {
                            PROGRESSBAR.as_ref().unwrap().finish();
                        }
                    }
                    _ => {}
                });

            success("> ALang Tools downloaded");

            info("> Installing..");

            if let Err(_) = fs::create_dir_all(format!("{}/tools", &data)) {
                err("Could not create tools folder");
            }

            #[cfg(windows)]
            let to_extract = format!("{}/downloads/tools_windows.zip", &data);

            #[cfg(target_os = "macos")]
            let to_extract = format!("{}/downloads/tools_macos.zip", &data);

            #[cfg(target_os = "linux")]
            let to_extract = format!("{}/downloads/tools_linux.zip", &data);

            let make_config = catch_unwind(|| {
                let now = SystemTime::now();
                let dur = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

                fs::write(format!("{}/updated", &data), format!("{}", dur)).unwrap();
            })
            .is_ok();

            #[cfg(windows)]
            let copy = fs::copy(
                format!("{}\\downloads\\cli_windows.exe", &data),
                format!("{}\\alang.exe", &data),
            )
            .is_ok()
                && make_config;

            #[cfg(target_os = "linux")]
            let copy = fs::copy(
                format!("{}/downloads/cli_linux", &data),
                format!("{}/alang", &data),
            )
            .is_ok()
                && make_config;

            #[cfg(target_os = "macos")]
            let copy = fs::copy(
                format!("{}/downloads/cli_macos", &data),
                format!("{}/alang", &data),
            )
            .is_ok()
                && make_config;

            if !copy {
                err("> Failed to install alang...");
            }

            let templates_folder = format!("{}/downloads/templates.zip", &data);
            let extract_dest = format!("{}/tools", &data);
            let temp_dest = format!("{}", &data);

            let ok = catch_unwind(move || {
                let zip_file = fs::File::open(to_extract).unwrap();
                let templates = fs::File::open(templates_folder).unwrap();

                let mut tools_zip = ZipArchive::new(zip_file).unwrap();
                let mut templates_zip = ZipArchive::new(templates).unwrap();

                tools_zip.extract(extract_dest).unwrap();
                templates_zip.extract(temp_dest).unwrap();
            })
            .is_ok();

            if !ok {
                err("> Failed to install alang tools...");
            }

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

            #[cfg(target_os = "linux")]
            {
                let res = catch_unwind(|| {
                    give_perms(data.clone());
                });

                if res.is_err() {
                    info(
                        format!("> Could not allow exec perms for the install <\n> Run `sudo chmod {}` <", &data).as_str(),
                    );
                }
            }

            println!("-------------------------------------------------------");

            if args.len() == 0 || args[0].as_str() != "-f" {
                info("Press enter key to exit");
                
                let mut buf = [0; 1];
                let _ = stdin().read_exact(&mut buf);

                process::exit(0);
            }
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
    red(data, true);
}

fn red(data: &str, crash: bool) {
    let mut chalk = Chalk::new();

    chalk.red().bold().println(&data);

    if crash {

        info("Please delete the existing files on the install dir\nPress enter key to exit");

        let mut buf = [0; 1];
        let _ = stdin().read_exact(&mut buf);

        process::exit(1);
    }
}

fn success(data: &str) {
    let mut chalk = Chalk::new();

    chalk.green().bold().println(&data);
}
