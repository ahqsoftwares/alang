use std::{fs, collections::HashMap, process::exit};
use indicatif::ProgressBar;
use serde_json::to_string_pretty;

use crate::{get_out, info, success, err, warn};

pub fn compile_code(dir: &String, base: &String) -> u64 {
    let out = format!("{}/app", get_out(&base));

    fs::create_dir_all(&out).unwrap_or(());
    
    let files = get_code_files(&dir);
    
    let mut file_map: HashMap<String, String> = HashMap::new();

    let pb = ProgressBar::new(files.len() as u64);

    let mut file_name = 0;
    for file in files {
        pb.inc(1);
        let parsed_file_name = file.replace(*&base, "").replace("/src/", "");

        pb.suspend(|| {
            info(format!("Building {}", &parsed_file_name));
        });

        file_name += 1;

        fs::write(
            format!("{}/{}.aasm", &out, &file_name),
            read_parse_file(file)
        ).unwrap();

        pb.suspend(|| {
            success(format!("Compiled {}", &parsed_file_name));
        });

        if file_map.insert(parsed_file_name.clone(), file_name.to_string()).is_some() {
            pb.finish_and_clear();
            err("Invalid File Name");
            warn("  At");
            warn(format!("  File: /src/{}", &parsed_file_name));
            exit(1);
        }
    }
    
    let file_map = to_string_pretty(&file_map).unwrap();
    fs::write(
        format!("{}/src.files.map.aasm.json", get_out(&base)),
        file_map,
    ).unwrap();

    pb.finish_and_clear();

    file_name
}

fn read_parse_file(file: String) -> String {
    fs::read_to_string(file)
        .unwrap()
        .split("\n")
        .map(|x| x.to_string().replace("\r", ""))
        .map(|x| x.to_string())
        .map(|x| x.trim().to_string())
        .filter(|x| x.len() > 0)
        .filter(|x| !x.starts_with("//"))
        .collect::<Vec<String>>()
        .join("\n")
        .split(";")
        .map(|x| x.to_string())
        .map(|x| {
            if *&x.starts_with("\n") {
                x.replacen("\n", "", 1)
            } else {
                x
            }
        })
        .map(|x| x.replace("{\n", "{").replace("}\n", "}"))
        .filter(|x| x.len() > 0)
        .collect::<Vec<String>>()
        .join(";")
}

fn get_code_files(dir: &String) -> Vec<String> {
    fs::read_dir(&dir)
        .unwrap()
        .map(|x| {
            let y = x.unwrap().file_name();

            y.to_str().unwrap().to_string()
        })
        .map(|x| format!("{}/{}", &dir, x))
        .collect()
}