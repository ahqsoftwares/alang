use inquire::Confirm;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use termimad::print_inline;

use crate::utils::info;

#[derive(Debug, Serialize, Deserialize)]
struct Release {
    body: String,
    tag_name: String,
}

pub fn show_notes() -> Option<()> {
    let client = Client::new();

    if let Ok(res) = client
        .get("https://api.github.com/repos/ahqsoftwares/alang/releases/latest")
        .header("User-Agent", "ALang/Cli")
        .send()
    {
        if let Ok(json) = res.json::<Release>() {
            info(format!("ALang v{} (Latest)", &json.tag_name.replace("lang-", "")).as_str());

            let data = format!("{}\n", &json.body);

            print_inline(data.as_str());

            if let Ok(option) = Confirm::new("Do you want to launch in browser?")
                .with_default(false)
                .prompt_skippable()
            {
                if let Some(req) = option {
                    if req {
                        None
                    } else {
                        Some(())
                    }
                } else {
                    Some(())
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
