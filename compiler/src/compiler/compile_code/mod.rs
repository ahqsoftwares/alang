use std::{collections::HashMap, hash::Hash};

use crate::InterpreterLog;

mod compile_code;
mod compile_variables;

use compile_variables::compile_variables;

pub fn compile(dir: &String) {
    let mut module_maps: InterpreterLog = HashMap::new();

    let variables = format!("{}/app", &dir);
    let code = format!("{}/src", &dir);

    compile_variables(&variables, &dir, &mut module_maps);
}
