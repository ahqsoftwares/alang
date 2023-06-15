use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::utils::{error, get_install_dir, info};

pub fn should_update() -> bool {
    let mut should_update = false;

    if let Some(dir) = get_install_dir() {
        let path = format!("{}/updated", dir);

        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(secs) = data.parse::<u64>() {
                let now = SystemTime::now();

                if let Ok(dur) = now.duration_since(UNIX_EPOCH) {
                    let now = dur.as_secs();
                    let last_checked = now - secs;

                    should_update = last_checked >= 24 * 60 * 60;

                    if should_update {
                        if let Err(_) = fs::write(&path, now.to_string()) {
                            info("WARN Unable to Update config file");
                        }
                    }
                } else {
                    error("Unable to compare time");
                }
            } else {
                error("Unable to parse ALang config");
            }
        } else {
            error("We were unable to read ALang config");
        }
    } else {
        error("We were unable to get ALang install dir");
    }

    should_update
}
