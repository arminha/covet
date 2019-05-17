/*
Copyright (C) 2019  Armin Häberling

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use clap::{App, AppSettings, Arg, SubCommand};

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum Source {
        auto,
        adf,
        glass
    }
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum Format {
        pdf,
        jpeg
    }
}

arg_enum! {
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
        .subcommand(
            SubCommand::with_name("status")
                .about("Display the status of the scanner")
                .arg(
                    Arg::with_name("SCANNER")
                        .help("The hostname of the scanner")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("no-tls")
                        .help("Do not use TLS to secure the connection to the scanner")
                        .long("no-tls"),
                ),
        )
        .subcommand(
            SubCommand::with_name("scan")
                .about("Scan a document")
                .arg(
                    Arg::with_name("SCANNER")
                        .help("The hostname of the scanner")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("SOURCE")
                        .help("The document source")
                        .takes_value(true)
                        .short("s")
                        .long("source")
                        .possible_values(&Source::variants())
                        .default_value("auto"),
                )
                .arg(
                    Arg::with_name("FORMAT")
                        .help("The format of the output")
                        .takes_value(true)
                        .short("f")
                        .long("format")
                        .possible_values(&Format::variants())
                        .default_value("pdf"),
                )
                .arg(
                    Arg::with_name("COLORSPACE")
                        .help("The color space of the output")
                        .takes_value(true)
                        .short("c")
                        .long("color")
                        .possible_values(&ColorSpace::variants())
                        .default_value("color"),
                )
                .arg(
                    Arg::with_name("RESOLUTION")
                        .help("The scan resolution in dpi")
                        .takes_value(true)
                        .short("r")
                        .long("resolution")
                        .possible_values(&["300", "600"])
                        .default_value("300"),
                )
                .arg(
                    Arg::with_name("QUALITY")
                        .help("Compression quality level (lower is better)")
                        .takes_value(true)
                        .short("q")
                        .long("compression-quality")
                        .default_value("25"),
                )
                .arg(
                    Arg::with_name("no-tls")
                        .help("Do not use TLS to secure the connection to the scanner")
                        .long("no-tls"),
                ),
        )
        .subcommand(
            SubCommand::with_name("web")
                .about("Start a web server to handle scan jobs")
                .arg(
                    Arg::with_name("SCANNER")
                        .help("The hostname of the scanner")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("PORT")
                        .help("Port to use for the web server")
                        .takes_value(true)
                        .short("p")
                        .long("port")
                        .default_value("3000"),
                )
                .arg(
                    Arg::with_name("ADDR")
                        .help("Listen address to use for the web server")
                        .takes_value(true)
                        .short("l")
                        .long("listen")
                        .default_value("127.0.0.1"),
                )
                .arg(
                    Arg::with_name("no-tls")
                        .help("Do not use TLS to secure the connection to the scanner")
                        .long("no-tls"),
                ),
        )
}
