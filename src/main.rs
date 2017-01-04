extern crate hyper;

mod scanstatus;
mod scanjob;

use hyper::client::{Client, Response};
use hyper::error::Result as HResult;
use hyper::Url;

use std::env;

use scanstatus::*;
use scanjob::*;

fn main() {
    let host = match env::args().nth(1) {
        Some(host) => host,
        None => {
            println!("Usage: covet <host>");
            return;
        }
    };

    let client = Client::new();
    print_scan_status(&client, &host);

    let job = ScanJob::new(InputSource::Platen, true, Format::Pdf, ColorSpace::Color);
    let mut target: Vec<u8> = Vec::new();
    job.write_xml(&mut target).unwrap();
    let result = String::from_utf8(target).unwrap();
    println!("{}", result);
}

fn print_scan_status(client: &Client, host: &str) {
    println!("Scan Status of {}", &host);
    let status = match get_scan_status(&client, &host) {
        Ok(status) => status,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("Scanner: {:?}, Adf: {:?}", status.get_scanner_state(), status.get_adf_state());
}

fn get_scan_status(client: &Client, host: &str) -> Result<ScanStatus, String> {
    retrieve_scan_status(&client, &host)
        .map_err(|e| e.to_string())
        .and_then(ScanStatus::read_xml)
}

fn retrieve_scan_status(client: &Client, host: &str) -> HResult<Response> {
    let url = "http://".to_string() + host + "/Scan/Status";
    let url = try!(Url::parse(&url));
    client.get(url).send()
}
