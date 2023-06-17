use std::{fs, process::Command};

pub fn give_perms(path: String) {
    let entries = fs::read_dir(&path).unwrap();

    entries.into_iter().for_each(|entry| {
        let entry = entry.unwrap();

        let data = entry.metadata().unwrap();

        if data.is_dir() {
            give_perms(format!("{}/{}", &path, entry.file_name().to_str().unwrap()));
        } else {
            Command::new("sudo")
                .arg("chmod")
                .arg("777")
                .arg(format!("{}/{}", &path, entry.file_name().to_str().unwrap()))
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    });
}
