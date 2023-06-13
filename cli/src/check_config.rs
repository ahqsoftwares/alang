use std::{fs, time::{SystemTime, UNIX_EPOCH}};

use crate::utils::{get_install_dir, error};

pub fn should_update() -> bool {
    let mut should_update = false;

    if let Some(dir) = get_install_dir() {
        if let Ok(data) = fs::read_to_string(format!("{}/updated", dir)) {
            if let Ok(secs) = data.parse::<u64>() {
                let now = SystemTime::now();

                if let Ok(dur) = now.duration_since(UNIX_EPOCH) {
                    let last_checked = dur.as_secs() - secs;

                    should_update = last_checked >= 24 * 60 * 60;
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