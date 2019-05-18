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
#![forbid(unsafe_code)]

use structopt::StructOpt;
use time;

mod cli;
mod message;
mod scanner;
mod web;

use std::thread;
use std::time::Duration;

use crate::cli::{Opt, ScannerOpt};
use crate::message::scan_job::{ColorSpace, Format, InputSource, ScanJob};
use crate::message::scan_status::AdfState;
use crate::scanner::{Scanner, ScannerError};

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Status(opt) => {
            status(opt);
        }
        Opt::Scan(opt) => {
            let use_tls = !opt.scanner_opts.no_tls;
            scan(
                &opt.scanner_opts.scanner,
                use_tls,
                opt.format.to_internal(),
                opt.color.to_internal(),
                opt.source,
                opt.resolution,
                opt.compression_quality,
            )
            .unwrap_or_else(|e| println!("Error: {}", e));
        }
        Opt::Web(opt) => {
            let use_tls = !opt.scanner_opts.no_tls;
            web::run_server(&opt.scanner_opts.scanner, &opt.listen, opt.port, use_tls);
        }
    }
}

fn status(opt: ScannerOpt) {
    let scanner = Scanner::new(&opt.scanner, !opt.no_tls);
    print_scan_status(&scanner).unwrap_or_else(|e| println!("Error: {}", e));
}

fn print_scan_status(scanner: &Scanner) -> Result<(), ScannerError> {
    println!("Status of scanner {}", scanner.host());
    let status = scanner.get_scan_status()?;
    println!(
        "Scanner: {:?}, Adf: {:?}",
        status.scanner_state(),
        status.adf_state()
    );
    Ok(())
}

impl cli::Format {
    fn to_internal(self) -> Format {
        match self {
            cli::Format::pdf => Format::Pdf,
            cli::Format::jpeg => Format::Jpeg,
        }
    }
}

impl cli::ColorSpace {
    fn to_internal(self) -> ColorSpace {
        match self {
            cli::ColorSpace::gray => ColorSpace::Gray,
            cli::ColorSpace::color => ColorSpace::Color,
        }
    }
}

fn choose_source(source: cli::Source, adf_state: AdfState) -> Result<InputSource, ScannerError> {
    let input_source = match source {
        cli::Source::auto => {
            if adf_state == AdfState::Loaded {
                InputSource::Adf
            } else {
                InputSource::Platen
            }
        }
        cli::Source::adf => {
            if adf_state == AdfState::Loaded {
                InputSource::Adf
            } else {
                return Err(ScannerError::AdfEmpty);
            }
        }
        cli::Source::glass => InputSource::Platen,
    };
    Ok(input_source)
}

fn scan(
    host: &str,
    use_tls: bool,
    format: Format,
    color: ColorSpace,
    source: cli::Source,
    resolution: u32,
    quality: u32,
) -> Result<(), ScannerError> {
    let scanner = Scanner::new(host, use_tls);
    let status = scanner.get_scan_status()?;
    if !status.is_idle() {
        return Err(ScannerError::Busy);
    }
    let input_source = choose_source(source, status.adf_state())?;
    let mut job = scanner.start_job(ScanJob::new(
        input_source,
        resolution,
        quality,
        format,
        color,
    ))?;
    println!("Job: {:?}", job);
    loop {
        let ready = job.retrieve_status()?;
        if ready {
            println!("Job: {:?}", job);
            let output_file = scanner::output_file_name(format, &time::now());
            job.download_to_file(&output_file)?;
            break;
        }
        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}
