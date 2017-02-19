#[macro_use]
extern crate clap;
extern crate time;
extern crate xml;
extern crate xmltree;

mod cli;
mod scanner;
mod message;
mod web;

use std::thread;
use std::time::Duration;

use message::job_status::PageState;
use message::scan_job::{ScanJob, InputSource, Format, ColorSpace};
use message::scan_status::AdfState;
use scanner::Scanner;

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
    } else if let Some(_) = matches.subcommand_matches("web") {
        web::run_server();
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

fn output_file_name(format: &Format, time: &time::Tm) -> String {
    let extension = match format {
        &Format::Pdf => "pdf",
        &Format::Jpeg => "jpeg"
    };
    let ts = time::strftime("%Y%m%d_%H%M%S", time).unwrap();
    format!("scan_{}.{}", ts, extension)
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
            let output_file = output_file_name(&format, &time::now());
            scanner.download(page.binary_url().unwrap(), &output_file).unwrap();
            break;
        }
        thread::sleep(Duration::from_millis(500));
    }
}

#[test]
fn check_output_file_name() {
    let time = time::at_utc(time::Timespec::new(1486905545, 0));
    assert_eq!("scan_20170212_131905.pdf", output_file_name(&Format::Pdf, &time));
    assert_eq!("scan_20170212_131905.jpeg", output_file_name(&Format::Jpeg, &time));
}
