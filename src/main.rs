extern crate hyper;

use hyper::Client;
use hyper::error::{Error, Result};
use hyper::Url;

use std::io::Read;
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
    let status = get_scan_status(&client, &host)
                    .unwrap_or_else(|err| "Error: ".to_string() + &err.to_string());
    println!("{}", &status);
}

fn get_scan_status(client: &Client, host: &str) -> Result<String> {
    let url = "http://".to_string() + host + "/Scan/Status";
    let url = try!(Url::parse(&url));
    let mut response = try!(client.get(url).send());
    let mut buffer = String::new();
    match response.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(Error::Io(e))
    }
}
