use inquire::{
    validator::{ErrorMessage, Validation},
    Text,
};

use crate::utils::error;

pub fn get_project_name() -> String {
    if let Ok(text) = Text::new("Project name")
        .with_validator(|data: &str| {
            let mut only_allowed: Vec<&str> =
                "qwertyuiopasdfghjklzxcvbnm123456789-_".split("").collect();
            only_allowed = only_allowed
                .iter()
                .filter(|a| a != &&"")
                .map(|x| *x)
                .collect();

            let data = data.clone().to_string();

            let data = data.chars().collect::<Vec<char>>();

            for ch in data {
                match only_allowed
                    .clone()
                    .iter()
                    .find(|a| *a.to_string() == ch.to_string())
                {
                    Some(_) => {}
                    _ => {
                        return Ok(Validation::Invalid(ErrorMessage::from(format!(
                            "'{}' is not allowed",
                            ch
                        ))))
                    }
                }
            }

            Ok(Validation::Valid)
        })
        .with_default("alang-project")
        .prompt()
    {
        text
    } else {
        error("Must Specify project name!");
        String::from("")
    }
}
