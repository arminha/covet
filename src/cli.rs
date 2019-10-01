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
use structopt::clap::{arg_enum, AppSettings};
use structopt::{self, StructOpt};

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy)]
    pub enum Source {
        auto,
        adf,
        glass
    }
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy)]
    pub enum Format {
        pdf,
        jpeg
    }
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy)]
    pub enum ColorSpace {
        gray,
        color
    }
}

#[derive(StructOpt, Debug)]
pub struct ScannerOpt {
    /// The hostname of the scanner
    #[structopt(name = "SCANNER")]
    pub scanner: String,

    /// Do not use TLS to secure the connection to the scanner
    #[structopt(long = "no-tls")]
    pub no_tls: bool,
}

#[derive(StructOpt, Debug)]
pub struct ScanOpt {
    #[structopt(flatten)]
    pub scanner_opts: ScannerOpt,

    /// The document source
    #[structopt(
        short,
        long,
        name = "SOURCE",
        default_value = "auto",
        possible_values(&Source::variants()),
        case_insensitive(true)
    )]
    pub source: Source,

    /// The format of the output
    #[structopt(
        short,
        long,
        name = "FORMAT",
        default_value = "pdf",
        possible_values(&Format::variants()),
        case_insensitive(true)
    )]
    pub format: Format,

    /// The color space of the output
    #[structopt(
        short,
        long,
        name = "COLORSPACE",
        default_value = "color",
        possible_values(&ColorSpace::variants()),
        case_insensitive(true)
    )]
    pub color: ColorSpace,

    /// The scan resolution in dpi
    #[structopt(
        short,
        long,
        name = "RESOLUTION",
        default_value = "300",
        possible_values(&["300", "600"])
    )]
    pub resolution: u32,

    /// Compression quality level (lower is better)
    #[structopt(
        short = "q",
        long = "compression-quality",
        name = "QUALITY",
        default_value = "25"
    )]
    pub compression_quality: u32,
}

#[derive(StructOpt, Debug)]
pub struct WebOpt {
    #[structopt(flatten)]
    pub scanner_opts: ScannerOpt,

    /// Port to use for the web server
    #[structopt(short, long, name = "PORT", default_value = "3000")]
    pub port: u16,

    /// Listen address to use for the web server
    #[structopt(short, long, name = "ADDR", default_value = "127.0.0.1")]
    pub listen: String,
}

#[derive(StructOpt, Debug)]
#[structopt(
    setting(AppSettings::VersionlessSubcommands),
    setting(AppSettings::InferSubcommands)
)]
pub enum Opt {
    /// Display the status of the scanner
    #[structopt(name = "status")]
    Status(ScannerOpt),

    /// Scan a document
    #[structopt(name = "scan")]
    Scan(ScanOpt),

    /// Start a web server to handle scan jobs
    #[structopt(name = "web")]
    Web(WebOpt),
}
