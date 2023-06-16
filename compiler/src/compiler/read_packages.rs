use crate::{get_out, Package, warn};
use serde_json::{from_str, to_string_pretty};

use std::{fs, path::Path};

pub fn get_package(dir: &String) -> Package {
    let path = get_out(&dir);

    let package_string = read_package(&dir);

    let splits = split_package(package_string);

    fs::create_dir_all(&path).unwrap_or(());

    remove_dir_contents(
        &path
    ).unwrap_or(());

    fs::create_dir_all(
        format!("{}/packages", &path)
    ).unwrap_or(());

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

        let first = string[0].clone().trim().to_string();
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
        } else {
            warn(
                format!("Unknown Package Syntax: {}", first)
            );
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
                .trim()
                .to_string()
        })
        .filter(|s| s != "")
        .collect()
}

fn remove_dir_contents<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            remove_dir_contents(&path)?;
            fs::remove_dir(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}