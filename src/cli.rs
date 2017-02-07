use clap::{Arg, App, AppSettings, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("covet")
            .version(crate_version!())
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::VersionlessSubcommands)
            .subcommand(SubCommand::with_name("status")
                .about("Display the status of the scanner")
                .arg(Arg::with_name("SCANNER")
                    .help("The hostname of the scanner")
                    .required(true)
                    .index(1)))
            .subcommand(SubCommand::with_name("scan")
                .about("Scan a document")
                .arg(Arg::with_name("SCANNER")
                    .help("The hostname of the scanner")
                    .required(true)
                    .index(1)))
}
