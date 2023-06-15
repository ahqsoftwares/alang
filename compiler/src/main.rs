use clap::{arg, command, error::ErrorKind};

mod compiler;
mod utils;
pub use utils::*;

use compiler::compile;

fn main() {
    let mut command = command!("compiler").arg(arg!(-w --workspace <FOLDER>));

    #[cfg(not(debug_assertions))]
    {
        command = command
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .disable_version_flag(true)
    }

    let matches = command.clone().get_matches();

    if !*&matches.args_present() {
        println!("Must specify an argument");
        std::process::exit(1);
    }

    if let Ok(arg) = *&matches.try_get_one::<String>("workspace") {
        if let Some(arg) = arg {
            compile(arg.clone());
        } else {
            command.error(ErrorKind::MissingRequiredArgument, "Workspace is required");
        }
    } else {
        command.error(ErrorKind::MissingRequiredArgument, "Workspace is required");
    }
}
