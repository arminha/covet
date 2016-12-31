extern crate hyper;
extern crate xmltree;

mod scanstatus;

use hyper::client::{Client, Response};
use hyper::error::Result as HResult;
use hyper::Url;

use xmltree::Element;

use std::env;
use std::io::Read;

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
    let status = match get_scan_status(&client, &host) {
        Ok(status) => status,
        Err(e) => {
            println!("Error: {}", &e);
            return;
        }
    };
    let status = parse_scan_status(status);
    println!("scanner: {:?}, adf: {:?}", status.get_scanner_state(), status.get_adf_state());
}

fn get_scan_status(client: &Client, host: &str) -> HResult<Response> {
    let url = "http://".to_string() + host + "/Scan/Status";
    let url = try!(Url::parse(&url));
    client.get(url).send()
}

fn parse_scan_status<R: Read>(r: R) -> ScanStatus {
    let element = Element::parse(r).unwrap();
    let scanner_state = element.get_child("ScannerState").unwrap().clone().text.unwrap();
    let scanner_state = ScannerState::parse(&scanner_state).unwrap();
    let adf_state = element.get_child("AdfState").unwrap().clone().text.unwrap();
    let adf_state = AdfState::parse(&adf_state).unwrap();
    ScanStatus::new(scanner_state, adf_state)
}

#[cfg(test)]
mod test {

    use scanstatus::{AdfState, ScannerState};

    const SCAN_STATUS_IDLE: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
            <ScanStatus xmlns=\"http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19\">\
            <ScannerState>Idle</ScannerState>\
            <AdfState>Empty</AdfState>\
            </ScanStatus>";

    const SCAN_STATUS_BUSY: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
            <ScanStatus xmlns=\"http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19\">\
            <ScannerState>BusyWithScanJob</ScannerState>\
            <AdfState>Empty</AdfState>\
            </ScanStatus>";

    const SCAN_STATUS_LOADED: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
            <ScanStatus xmlns=\"http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19\">\
            <ScannerState>Idle</ScannerState>\
            <AdfState>Loaded</AdfState>\
            </ScanStatus>";

    #[test]
    fn parse_scan_status() {
        fn check_parse_scan_status(s: &str, scanner_state: ScannerState, adf_state: AdfState) {
            let status = s.as_bytes();
            let scan_status = super::parse_scan_status(status);
            assert_eq!(scanner_state, scan_status.get_scanner_state());
            assert_eq!(adf_state, scan_status.get_adf_state());
        }
        check_parse_scan_status(SCAN_STATUS_IDLE, ScannerState::Idle, AdfState::Empty);
        check_parse_scan_status(SCAN_STATUS_BUSY, ScannerState::BusyWithScanJob, AdfState::Empty);
        check_parse_scan_status(SCAN_STATUS_LOADED, ScannerState::Idle, AdfState::Loaded);
    }

}
