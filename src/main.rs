extern crate hyper;

use hyper::Client;
use hyper::error::Result;
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
    let status = get_scan_status(&client, &host).unwrap();
    println!("{}", &status);
}

fn get_scan_status(client: &Client, host: &str) -> Result<String> {
    let url = "http://".to_string() + host + "/Scan/Status";
    let url = Url::parse(&url).unwrap();
    let mut response = client.get(url).send().unwrap();
    let mut buffer = String::new();
    response.read_to_string(&mut buffer).unwrap();
    Ok(buffer)
}
