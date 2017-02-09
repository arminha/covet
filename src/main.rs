#[macro_use]
extern crate clap;

mod job_status;
mod scan_status;
mod scan_job;
mod scanner;
mod cli;

use std::{thread, time};

use job_status::PageState;
use scanner::Scanner;
use scan_job::{ScanJob, InputSource, Format, ColorSpace};
use scan_status::AdfState;

fn main() {
    let matches = cli::build_cli().get_matches();
    if let Some(matches) = matches.subcommand_matches("status") {
        let host = matches.value_of("SCANNER").unwrap();
        status(host);
    } else if let Some(matches) = matches.subcommand_matches("scan") {
        let host = matches.value_of("SCANNER").unwrap();
        let format = value_t!(matches.value_of("FORMAT"), cli::Format).unwrap();
        let color = value_t!(matches.value_of("COLORSPACE"), cli::ColorSpace).unwrap();
        let source = value_t!(matches.value_of("SOURCE"), cli::Source).unwrap();
        let resolution = value_t!(matches.value_of("RESOLUTION"), u32).unwrap();
        scan(host, format.to_internal(), color.to_internal(), source, resolution);
    }
}

fn status(host: &str) {
    let scanner = Scanner::new(host);
    print_scan_status(&scanner);
}

impl cli::Format {
    fn to_internal(&self) -> Format {
        match self {
            &cli::Format::pdf => Format::Pdf,
            &cli::Format::jpeg => Format::Jpeg
        }
    }
}

impl cli::ColorSpace {
    fn to_internal(&self) -> ColorSpace {
        match self {
            &cli::ColorSpace::gray => ColorSpace::Gray,
            &cli::ColorSpace::color => ColorSpace::Color
        }
    }
}

fn print_scan_status(scanner: &Scanner) {
    println!("Status of scanner {}", scanner.host());
    let status = match scanner.get_scan_status() {
        Ok(status) => status,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("Scanner: {:?}, Adf: {:?}", status.scanner_state(), status.adf_state());
}

fn scan(host: &str, format: Format, color: ColorSpace, source: cli::Source, resolution:u32) {
    let scanner = Scanner::new(host);
    let status = match scanner.get_scan_status() {
        Ok(status) => status,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    if status.is_busy() {
        println!("Scanner is busy");
    }
    let input_source = match source {
        cli::Source::auto => {
            if status.adf_state() == AdfState::Loaded {
                InputSource::Adf
            } else {
                InputSource::Platen
            }
        },
        cli::Source::adf => {
            if status.adf_state() == AdfState::Loaded {
                InputSource::Adf
            } else {
                println!("Adf is empty");
                return;
            }
        },
        cli::Source::glass => InputSource::Platen
    };
    let job = ScanJob::new(input_source, resolution, format, color);
    let job_location = match scanner.start_job(job) {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to start scanjob: {}", e);
            return;
        }
    };
    println!("Job Location: {}", job_location);
    loop {
        let status = scanner.get_job_status(&job_location).expect("no job status");
        println!("{:?}", status);
        let page = status.pages().get(0).unwrap();
        let page_state = page.state();
        if page_state == PageState::ReadyToUpload {
            println!("http://{}{}", scanner.host(), page.binary_url().unwrap());
            scanner.download(page.binary_url().unwrap(), "test.pdf").unwrap();
            break;
        }
        thread::sleep(time::Duration::from_millis(500));
    }
}
