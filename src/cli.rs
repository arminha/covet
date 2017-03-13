use clap::{Arg, App, AppSettings, SubCommand};

arg_enum!{
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum Source {
        auto,
        adf,
        glass
    }
}

arg_enum!{
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum Format {
        pdf,
        jpeg
    }
}

arg_enum!{
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum ColorSpace {
        gray,
        color
    }
}

pub fn build_cli() -> App<'static, 'static> {
    App::new("covet")
            .version(crate_version!())
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::VersionlessSubcommands)
            .setting(AppSettings::InferSubcommands)
            .subcommand(SubCommand::with_name("status")
                .about("Display the status of the scanner")
                .arg(Arg::with_name("SCANNER")
                    .help("The hostname of the scanner")
                    .required(true)
                    .index(1))
                )
            .subcommand(SubCommand::with_name("scan")
                .about("Scan a document")
                .arg(Arg::with_name("SCANNER")
                    .help("The hostname of the scanner")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("SOURCE")
                    .help("The document source")
                    .takes_value(true)
                    .short("s")
                    .long("source")
                    .possible_values(&Source::variants())
                    .default_value("auto"))
                .arg(Arg::with_name("FORMAT")
                    .help("The format of the output")
                    .takes_value(true)
                    .short("f")
                    .long("format")
                    .possible_values(&Format::variants())
                    .default_value("pdf"))
                .arg(Arg::with_name("COLORSPACE")
                    .help("The color space of the output")
                    .takes_value(true)
                    .short("c")
                    .long("color")
                    .possible_values(&ColorSpace::variants())
                    .default_value("color"))
                .arg(Arg::with_name("RESOLUTION")
                    .help("The scan resolution in dpi")
                    .takes_value(true)
                    .short("r")
                    .long("resolution")
                    .possible_values(&["300", "600"])
                    .default_value("300"))
                )
            .subcommand(SubCommand::with_name("web")
                .about("Start a web interface to handle scan jobs")
                .arg(Arg::with_name("SCANNER")
                    .help("The hostname of the scanner")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("PORT")
                    .help("Port to use for the webserver")
                    .takes_value(true)
                    .short("p")
                    .long("port")
                    .default_value("3000"))
                )
}
