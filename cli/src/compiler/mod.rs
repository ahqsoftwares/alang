use std::{
    fs::{self, DirEntry},
    process::Command,
};

use crate::utils::{error, get_install_dir, info};

pub fn compile_code(workspace: String, verboose: bool) {
    if verboose {
        println!("> Validating Dir");
    }

    if let Ok(dir) = fs::read_dir(&workspace) {
        let len = dir
            .into_iter()
            .map(|x| {
                if let Ok(x) = x {
                    x
                } else {
                    error("Invalid Workspace");
                    panic!();
                }
            })
            .collect::<Vec<DirEntry>>()
            .len();

        if verboose {
            println!("> Validating Workspace");
        }

        if len == 0 {
            error("Invalid Workspace");
        }

        let dir = get_install_dir();

        if let None = &dir {
            error("Install Dir Not Found");
        }

        let dir = dir.unwrap();

        #[cfg(not(windows))]
        let compiler = format!("{}/tools/compiler", &dir);

        #[cfg(windows)]
        let compiler = format!("{}/tools/compiler.exe", &dir);

        if verboose {
            print!("> Running ");
            info(format!("`{} -w {}`", &compiler, &workspace).as_str());
        }

        if let Ok(child) = &mut Command::new(&compiler)
            .arg("-w")
            .arg(format!("{}", &workspace))
            .spawn()
        {
            if let Ok(exit) = child.wait() {
                if !exit.success() {
                    error("Compiler failed to succeed");
                }
                if verboose {
                    println!("> Compiler exited with code {}", exit.code().unwrap());
                }
            } else {
                error("Unable to run compiler");
            }
        } else {
            error("Unable to run compiler");
        }
    } else {
        error("Workspace Not Found");
    }
}
