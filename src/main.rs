extern crate hyper;

use hyper::Client;
use hyper::Url;

use std::io::Read;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Downloading {}", args[1]);
    let url = Url::parse(&args[1]).unwrap();
    let client = Client::new();
    let mut res = client.get(url).send().unwrap();
    println!("{:?}", res);
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();
    println!("{}", buffer);
}
