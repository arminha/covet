extern crate hyper;

mod scanstatus;

use hyper::client::{Client, Response};
use hyper::error::Result as HResult;
use hyper::Url;

use std::env;

use scanstatus::*;

fn main() {
    let host = match env::args().nth(1) {
        Some(host) => host,
        None => {
            println!("Usage: covet <host>");
            return;
        }
    };

    println!("Scan Status of {}", &host);
    let client = Client::new();
    let xml = match get_scan_status(&client, &host) {
        Ok(xml) => xml,
        Err(e) => {
            println!("Error: {}", &e);
            return;
        }
    };
    let status = match ScanStatus::read_xml(xml) {
        Ok(status) => status,
        Err(e) => {
            println!("Error: Could not parse XML: {}", e);
            return;
        }
    };
    println!("scanner: {:?}, adf: {:?}", status.get_scanner_state(), status.get_adf_state());
}

fn get_scan_status(client: &Client, host: &str) -> HResult<Response> {
    let url = "http://".to_string() + host + "/Scan/Status";
    let url = try!(Url::parse(&url));
    client.get(url).send()
}
