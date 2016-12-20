extern crate hyper;
extern crate xmltree;

use hyper::client::{Client, Response};
use hyper::error::Result;
use hyper::Url;

use xmltree::Element;

use std::env;

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
    let status = match get_scan_status(&client, &host) {
        Ok(status) => status,
        Err(e) => {
            println!("Error: {}", &e);
            return;
        }
    };
    let element = Element::parse(status).unwrap();
    let scanner_state = element.get_child("ScannerState").unwrap().clone().text.unwrap();
    let adf_state = element.get_child("AdfState").unwrap().clone().text.unwrap();
    println!("scanner: {}, adf: {}", &scanner_state, &adf_state);
}

fn get_scan_status(client: &Client, host: &str) -> Result<Response> {
    let url = "http://".to_string() + host + "/Scan/Status";
    let url = try!(Url::parse(&url));
    client.get(url).send()
}
