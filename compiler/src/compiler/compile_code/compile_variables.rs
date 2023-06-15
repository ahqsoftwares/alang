use std::fs;

use crate::{get_out, InterpreterLog};

pub fn compile_variables(dir: &String, base: &String, _log: &mut InterpreterLog) {
    fs::create_dir_all(format!("{}/variables", get_out(&base))).unwrap_or(());

    let files = get_variable_files(&dir);

    for file in files {
        let data = read_files(file);

        println!("{:#?}", data);
    }
}

fn read_files(file: String) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap()
        .split("\n")
        .map(|x| x.to_string().replace("\r", ""))
        .map(|x| x.to_string())
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
        .filter(|x| x.len() > 0)
        .collect()
}

fn get_variable_files(dir: &String) -> Vec<String> {
    fs::read_dir(&dir)
        .unwrap()
        .map(|x| {
            let y = x.unwrap().file_name();

            y.to_str().unwrap().to_string()
        })
        .map(|x| format!("{}/{}", &dir, x))
        .collect()
}
