use std::panic::catch_unwind;
use chalk_rs::Chalk;
use reqwest::blocking::Client;

use crate::{check_config::should_update, utils::ALangRelease};

pub fn check_update() {
    if should_update() {
        let err = catch_unwind(move || {
            let version: String = env!("CARGO_PKG_VERSION").to_string();
            let client = Client::new();

            let release = client.get("https://api.github.com/repos/ahqsoftwares/alang/releases/latest")
                .header("User-Agent", "Alang-Updater/Cli")
                .send()
                .unwrap()
                .json::<ALangRelease>()
                .unwrap();

            if &release.tag_name.replace("lang-", "") != &version {
                let mut chalk = Chalk::new();

                chalk.blue().bold().println(
                    &format!("A new version of ALang Cli is available\nv{} is available to download\nRun `alang update` to install", &release.tag_name.replace("lang-", "")).as_str()
                );
            }
        }).is_err();

        if err {
            let mut chalk = Chalk::new();

            chalk.yellow().bold().println(
                &"Something went wrong fetching update data"
            );
        }
    }
}
