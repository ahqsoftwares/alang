use serde::{Deserialize, Serialize};

use reqwest::blocking::Client;

static UA: &str = "ALang/Installer";

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
pub enum AssetClass {
    WindowsTools,
    LinuxTools,
    MacosTools,
    LinuxCli,
    MacosCli,
    WindowsCli,
    CodeTemplates,
}

#[derive(Debug)]
pub struct ParsedAsset {
    pub class: AssetClass,
    pub url: String,
}

#[derive(Debug)]
pub struct Urls {
    pub version: String,
    pub assets: Vec<ParsedAsset>,
}

impl Urls {
    fn default() -> Self {
        Self {
            version: String::from("v0"),
            assets: vec![],
        }
    }

    fn set_version(&mut self, version: &String) {
        self.version = version.clone();
    }

    fn push(&mut self, asset: ParsedAsset) {
        self.assets.push(asset);
    }
}

pub fn get_urls() -> Urls {
    let client = Client::new();

    let mut url = Urls::default();

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

        //ignore the installers & updaters
        if &asset.name.contains("installer") == &false {
            url.push(ParsedAsset {
                class: (|| {
                    let name = asset.name.clone();
                    let name = name.as_str();

                    if name == "templates.zip" {
                        AssetClass::CodeTemplates
                    } else if name == "tools-windows.zip" {
                        AssetClass::WindowsTools
                    } else if name == "tools-linux.zip" {
                        AssetClass::LinuxTools
                    } else if name == "tools-macos.zip" {
                        AssetClass::MacosTools
                    } else if name == "cli_linux" {
                        AssetClass::LinuxCli
                    } else if name == "cli_windows.exe" {
                        AssetClass::WindowsCli
                    } else if name == "cli_macos" {
                        AssetClass::MacosCli
                    } else {
                        panic!("Unknown asset: {}", &asset.name);
                    }
                })(),
                url: asset.browser_download_url.clone(),
            });
        }
    });

    url
}
