use crate::{get_out, Package};
use serde_json::{from_str, to_string_pretty};

use std::fs;

pub fn get_package(dir: &String) -> Package {
    let path = get_out(&dir);

    fs::create_dir_all(&path).unwrap_or(());

    let package_string = read_package(&dir);

    let splits = split_package(package_string);

    read(splits, &path)
}

fn read(splits: Vec<String>, dir: &String) -> Package {
    let mut package = Package::default();

    for split in splits {
        let string = split
            .clone()
            .split(" ")
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        let first = string[0].clone();
        let first = first.as_str();

        let second = string.join(" ").replace(&first, "").clone();

        if first == "name" {
            package.name = Some(second.replacen(" ", "", 1));
        } else if first == "version" {
            package.version = Some(second.replacen(" ", "", 1));
        } else if first == "description" {
            package.description = Some(second.replacen(" ", "", 1));
        } else if first == "author" {
            package.author = Some(second.replacen(" ", "", 1));
        } else if first == "license" {
            package.license = Some(second.replacen(" ", "", 1));
        } else if first == ".packages" {
            package.packages = Some(from_str(second.as_str()).unwrap());
        } else if first == ".versions" {
            package.package_map = Some(from_str(second.as_str()).unwrap());
        } else if first == ".modules" {
            package.modules = Some(from_str(second.as_str()).unwrap());
        } else if first == ".config" {
            package.config = Some(from_str(second.as_str()).unwrap());
        }
    }

    let json = to_string_pretty(&package).unwrap();
    fs::write(format!("{}/package.aasm.json", dir), json).unwrap();

    package
}

fn read_package(dir: &String) -> String {
    fs::read_to_string(format!("{}/packages.css", dir)).unwrap()
}

fn split_package(package_string: String) -> Vec<String> {
    package_string
        .split(";")
        .map(ToString::to_string)
        .map(|s| {
            s.replace("\n", "")
                .replace("\r", "")
                /* Replaces Legacy '@' */
                .replace("@", "")
        })
        .filter(|s| s != "")
        .collect()
}
