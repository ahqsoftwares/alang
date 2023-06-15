use reqwest::blocking::Client;

use crate::{check_config::should_update, utils::ALangRelease};

pub fn check_update() -> bool {
    if should_update() {
        let version: String = env!("CARGO_PKG_VERSION").to_string();
        let client = Client::new();

        let release = client
            .get("https://api.github.com/repos/ahqsoftwares/alang/releases/latest")
            .header("User-Agent", "Alang-Updater/Cli")
            .send()
            .unwrap()
            .json::<ALangRelease>()
            .unwrap();

        if &release.tag_name.replace("lang-", "") != &version {
            true
        } else {
            false
        }
    } else {
        false
    }
}
