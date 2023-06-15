use clap::{command, arg, error::ErrorKind};

fn main() {
    let mut command = command!("packloader")
        .arg(arg!(-w --workspace <FOLDER>));

    #[cfg(not(debug_assertions))]
    {
        command = command.disable_help_flag(true)
                    .disable_help_subcommand(true)
                    .disable_version_flag(true)
    }

    let matches = command.clone().get_matches();

    if !*&matches.args_present() {
        println!("Must specify an argument");
        std::process::exit(1);
    }

    if let Ok(arg) = *&matches.try_get_one::<String>("workspace") {
        if let Some(_arg) = arg {
        } else {
            command.error(ErrorKind::MissingRequiredArgument, "Workspace is required");
        }
    } else {
        command.error(ErrorKind::MissingRequiredArgument, "Workspace is required");
    }
}
