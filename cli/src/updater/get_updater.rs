use serde::{Deserialize, Serialize};

use reqwest::blocking::Client;

static UA: &str = "ALang/Updater";

#[derive(Serialize, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Serialize, Deserialize)]
struct ALangRelease {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Debug)]
pub struct Url {
    pub version: String,
    pub asset_url: String,
}

impl Url {
    fn default() -> Self {
        Self {
            version: String::from("v0"),
            asset_url: String::from(""),
        }
    }

    fn set_version(&mut self, version: &String) {
        self.version = version.clone();
    }

    fn set_asset(&mut self, asset: String) {
        self.asset_url = asset;
    }
}

pub fn get_url() -> Url {
    let client = Client::new();

    let mut url = Url::default();

    let assets = client
        .get("https://api.github.com/repos/ahqsoftwares/alang/releases/latest")
        .header("User-Agent", UA.clone())
        .send()
        .unwrap()
        .json::<ALangRelease>()
        .unwrap();

    url.set_version(&assets.tag_name.replace("lang-", ""));

    assets.assets.iter().for_each(|asset| {
        let asset = asset.clone().to_owned();

        #[cfg(windows)]
        let updater = "alang_installer_windows.exe";

        #[cfg(target_os = "macos")]
        let updater = "alang_installer_macos";

        #[cfg(target_os = "linux")]
        let updater = "alang_installer_linux";

        if &asset.name.starts_with(updater) == &true {
            url.set_asset(asset.browser_download_url.clone());
        }
    });

    url
}
