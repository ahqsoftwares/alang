use clap::{arg, crate_version, Command as CliCommand};

static mut VER: Option<String> = None;

fn main() {
    unsafe {
        VER = Some(format!("v{}", crate_version!()));
    }

    let command = unsafe {
        CliCommand::new("alang")
            .version(&VER.as_ref().unwrap().as_str())
            .subcommand(
                CliCommand::new("version")
                    .about("Print version")
                    .alias("ver"),
            )
            .subcommand(
                CliCommand::new("init")
                .about("Initialize an ALang project inside the current dir")
                .arg_required_else_help(true),
            )
            .subcommand(
                CliCommand::new("new")
                    .about("Initialize an ALang project inside the current dir")
                    .alias("create")
                    .arg(arg!([DIR]))
                    .arg_required_else_help(true),
            )
    };

    let mut cloned_cmd = command.clone();

    let matches = command.get_matches();

    if let Some(_) = matches.subcommand_matches("version") {
        let data = &cloned_cmd.get_version().unwrap();

        println!("ALang Cli v{}", &data);
    } else if let Some(_) = matches.subcommand_matches("init") {
        
    } else {
        let _ = &cloned_cmd.print_help();
    }
}
