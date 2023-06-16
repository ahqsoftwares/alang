use std::fs;

use inquire::Select;

use crate::utils::{error, get_install_dir};

use super::err_reading_dir;

pub fn get_template() -> String {
    let dir = get_install_dir().unwrap();

    if let Ok(dir) = fs::read_dir(format!("{}/templates", &dir)) {
        let templates: Vec<String> = dir.map(|s| {
            let entry = s.unwrap();

            if &entry.metadata().unwrap().is_dir() == &false {
                panic!("Invalid template dir");
            }

            entry.file_name().to_str().unwrap().to_string()
        })
        .collect();

        if let Ok(template) = Select::new("Select template", templates).prompt() {
            template
        } else {
            error("You must select a template");
            "".to_string()
        }
    } else {
        err_reading_dir();
        "".to_string()
    }
}

pub fn install_template(dir: String, template: String) {
    let template_dir = format!("{}/templates/{}", get_install_dir().unwrap(), &template);

    copy_dir(template_dir, dir);
}

struct DirData {
    is_dir: bool,
    file_name: String
}

fn copy_dir(a: String, dest: String) {
    fs::create_dir_all(&dest).unwrap();

    if let Ok(dir) = fs::read_dir(&a) {
        let dir_data = dir.map(|entry| {
            if let Ok(entry) = entry {
                if let (Ok(meta), Some(file)) = (&entry.metadata(), &entry.file_name().to_str()) {
                    DirData {
                        file_name: file.to_string(),
                        is_dir: meta.is_dir(),
                    }
                } else {
                    err_reading_dir();
                    DirData {
                        file_name: "".to_owned(),
                        is_dir: false
                    }
                }
            } else {
                err_reading_dir();
                DirData {
                    file_name: "".to_owned(),
                    is_dir: false
                }
            }
        }).collect::<Vec<DirData>>();

        for entry in dir_data {
            if entry.is_dir {
                copy_dir(
                    format!("{}/{}", &a, &entry.file_name),
                    format!("{}/{}", &dest, &entry.file_name)
                );
            } else {
                let path = format!("{}/{}", &a, &entry.file_name);
                let dest_file = format!("{}/{}", &dest, &entry.file_name);

                if let Err(e) = fs::copy(
                    &path,
                    &dest_file
                ) {
                    error("Could not populate the directory!");
                }
            }
        }
    } else {
        err_reading_dir();
    }
}