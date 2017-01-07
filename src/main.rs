mod scan_status;
mod scan_job;
mod scanner;

use std::env;

use scanner::Scanner;
use scan_job::{ScanJob, InputSource, Format, ColorSpace};

fn main() {
    let host = match env::args().nth(1) {
        Some(host) => host,
        None => {
            println!("Usage: covet <host>");
            return;
        }
    };

    let scanner = Scanner::new(&host);
    print_scan_status(&scanner);
    create_job(&scanner);
}

fn print_scan_status(scanner: &Scanner) {
    println!("Scan Status of {}", scanner.host());
    let status = match scanner.get_scan_status() {
        Ok(status) => status,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("Scanner: {:?}, Adf: {:?}", status.scanner_state(), status.adf_state());
}

fn create_job(scanner: &Scanner) {
    let job = ScanJob::new(InputSource::Platen, false, Format::Pdf, ColorSpace::Gray);
    let job_location = match scanner.start_job(job) {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to start scanjob: {}", e);
            return;
        }
    };
    println!("Job Location: {}", job_location);
}
