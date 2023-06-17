mod download;
mod get_updater;

use dirs::download_dir;
use get_updater::get_url;
use std::process::Command;

use crate::utils::{error, get_install_dir, info};

static mut VERSION: Option<String> = None;

pub fn update() {
    let url = get_url();

    unsafe {
        VERSION = Some(url.version.clone());
    }

    if let Some(downloads) = download_dir() {
        if let Some(downloads) = downloads.to_str() {
            unsafe {
                let installer_path_str = format!(
                    "{}/installer_{}{}",
                    &downloads,
                    VERSION.as_ref().unwrap_or(&"".to_string()),
                    if cfg!(windows) { ".exe" } else { "" }
                );

                download::download(
                    url.asset_url.clone(),
                    format!("{}", &downloads),
                    format!(
                        "installer_{}{}",
                        VERSION.as_ref().unwrap_or(&"".to_string()),
                        if cfg!(windows) { ".exe" } else { "" }
                    ),
                    |c, t| {
                        println!(
                            "{}% downloaded v{}",
                            if (c * 100) / t >= 100 {
                                "100".to_string()
                            } else {
                                format!(" {}", (c * 100) / t)
                            },
                            VERSION.as_ref().unwrap_or(&"".to_string())
                        );
                    },
                );

                #[cfg(not(target_os = "macos"))]
                {
                    info("Running Updater, It is advised to delete the temporary installer file in your downloads folder");

                    #[cfg(windows)]
                    let command = Command::new("powershell")
                        .arg("start-process")
                        .arg("-FilePath")
                        .arg(installer_path_str)
                        .arg("-ArgumentList")
                        .arg(format!("\"-f {}\"", get_install_dir().unwrap()))
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap()
                        .success();

                    #[cfg(not(windows))]
                    let command = Command::new("bash")
                        .arg("-c")
                        .arg(format!("{} -f \"{}\" &", installer_path_str, get_install_dir().unwrap()))
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap()
                        .success();

                    if !command {
                        error("Failed to run installer");
                    }
                }

                #[cfg(target_os = "macos")]
                info("Kindly run the updater placed in your downloads folder");
            }
        } else {
            error("Downloads directory not UTF-8");
        }
    } else {
        error("Downloads directory not found");
    }
}
