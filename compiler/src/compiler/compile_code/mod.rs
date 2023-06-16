mod compile_code;
mod compile_variables;

use compile_variables::compile_variables;

use crate::{info, success};

use self::compile_code::compile_code;

pub fn compile(dir: &String) {
    info("Transforming Variables");

    let variables = format!("{}/app", &dir);

    let code = format!("{}/src", &dir);

    let total = compile_variables(&variables, &dir);

    success(
        format!("Transformed Variables ({})", &total)
    );

    info("Compiling Code");

    let total = compile_code(&code, &dir);

    success(
        format!("Compiled Code ({})", &total)
    );
}
