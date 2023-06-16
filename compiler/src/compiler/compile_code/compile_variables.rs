use std::{fs, collections::HashMap, process::exit};

use serde_json::{from_str, to_string, Value, to_string_pretty};

use crate::{get_out, Variable, Var, VariableValue, err, warn};

use indicatif::ProgressBar;

pub fn compile_variables(dir: &String, base: &String) -> u64 {
    fs::create_dir_all(format!("{}/variables", get_out(&base))).unwrap_or(());

    let files = get_variable_files(&dir);

    let mut file_name = 0;

    let mut map: HashMap<String, String> = HashMap::new();

    let pb = ProgressBar::new(files.len() as u64);

    for file in files {
        let parsed_file_name = file.replace(*&base, "").replace("/app/", "");

        if !parsed_file_name.starts_with("variable.") {
            err("Only Variable Files (variable.{x}.css) format is allowed");
            warn("Trace");
            warn(format!("At File: app/{}", &parsed_file_name));
            exit(1);
        }

        pb.inc(1);
        file_name += 1;

        let mut variables: Vec<Variable> = vec![];

        let data = read_file(file.clone());

        for slice in data {
            let splits = slice.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();

            let first = splits[0].as_str();
            let second = splits.join(" ").replace(&first, "").replacen(" ", "", 1);

            if first == ".global" {
                from_str::<Vec<String>>(second.as_str())
                    .unwrap()
                    .iter()
                    .for_each(|item| {
                        let item = item.clone();

                        let dup = variables.iter().find(|x| &x.name == &item).is_some();

                        if dup {
                            err(
                                format!("Found Duplicate Variable: {}", &item)
                            );
                            warn("Trace");
                            warn(format!("At File: app/{}", &parsed_file_name));
                            warn(format!("Variable Flagged: {}", &item));
                            exit(1);
                        }

                        variables.push(Variable {
                            name: item,
                            value: VariableValue::None,
                            var_type: Var::Global
                        });
                    });
            } else if first == ".stated" {
                from_str::<HashMap<String, Value>>(second.as_str())
                    .unwrap()
                    .iter()
                    .for_each(|item| {
                        let item = item.clone();

                        let key = item.0.clone();
                        let dup = variables.iter().find(|x| &x.name == &key).is_some();

                        if dup {
                            err(
                                format!("Found Duplicate Variable: {}", item.0.to_string())
                            );
                            warn("Trace");
                            warn(format!("At File: app/{}", &parsed_file_name));
                            warn(format!("Variable Flagged: {}", &first));
                            exit(1);
                        }

                        variables.push(Variable {
                            name: item.0.to_string(),
                            value: VariableValue::Val(item.1.clone()),
                            var_type: Var::Stated
                        });
                    });
            } else if first == ".constants" {
                from_str::<HashMap<String, Value>>(second.as_str())
                    .unwrap()
                    .iter()
                    .for_each(|item| {
                        let item = item.clone();

                        let dup = variables.iter().find(|x| &x.name == item.0).is_some();

                        if dup {
                            err(
                                format!("Found Duplicate Variable: {}", &item.0)
                            );
                            warn("Trace");
                            warn(format!("At File: app/{}", &parsed_file_name));
                            warn(format!("Variable Flagged: {}", &item.0));
                            exit(1);
                        }

                        variables.push(Variable {
                            name: item.0.to_string(),
                            value: VariableValue::Val(item.1.clone()),
                            var_type: Var::Constant
                        });
                    });
            } else {
                pb.suspend(|| {
                    warn(
                        format!("Unknown Signature: {}", &first)
                    );
                    warn(
                        format!("   At File: app/{}", &parsed_file_name)
                    );
                });
            }
        }

        let mut errors: Vec<String> = vec![];
        variables.clone().iter().for_each(|x| {
            let allowed = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890".split("").map(|x| x.to_string()).collect::<Vec<String>>();

            let chars =  x.name.chars();

            for character in chars {
                if !allowed.contains(&character.to_string()) {
                    errors.push(x.name.clone());
                }
            }
        });

        if errors.len() > 0 {
            err(format!("Found Invalid Variable Name(s): {}", errors.join(", ")));
            exit(1);
        }

        fs::write(
            format!("{}/variables/{}.json", get_out(&base), &file_name),
            to_string(&variables).unwrap()
        ).unwrap();

        if map.insert(
            parsed_file_name, 
            file_name.to_string()
        ).is_some() {
            err("Duplicate Variable Found..");
            exit(1);
        }
    }

    pb.finish_and_clear();

    fs::write(
        format!("{}/variables.map.aasm.json", get_out(&base)),
        to_string_pretty(&map).unwrap()
    ).unwrap();

    file_name
}

fn read_file(file: String) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap()
        .split("\n")
        .map(|x| x.replace("{{;}}", "ॎ"))
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
        .map(|x| x.replace("ॎ", ";"))
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
