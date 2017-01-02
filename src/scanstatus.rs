extern crate xmltree;

use self::xmltree::Element;

use std::io::Read;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScannerState {
    Idle,
    BusyWithScanJob,
}

impl ScannerState {
    pub fn parse(s: &str) -> Result<ScannerState, ()> {
        match s {
            "Idle" => Ok(ScannerState::Idle),
            "BusyWithScanJob" => Ok(ScannerState::BusyWithScanJob),
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AdfState {
    Empty,
    Loaded,
}

impl AdfState {
    pub fn parse(s: &str) -> Result<AdfState, ()> {
        match s {
            "Empty" => Ok(AdfState::Empty),
            "Loaded" => Ok(AdfState::Loaded),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub struct ScanStatus {
    scanner_state: ScannerState,
    adf_state: AdfState,
}

impl ScanStatus {
    pub fn new(scanner_state: ScannerState, adf_state: AdfState) -> ScanStatus {
        ScanStatus { scanner_state: scanner_state, adf_state: adf_state }
    }

    pub fn get_scanner_state(&self) -> ScannerState {
        self.scanner_state
    }

    pub fn get_adf_state(&self) -> AdfState {
        self.adf_state
    }

    pub fn read_xml<R: Read>(r: R) -> ScanStatus {
        let element = Element::parse(r).unwrap();
        let scanner_state = element.get_child("ScannerState").unwrap().clone().text.unwrap();
        let scanner_state = ScannerState::parse(&scanner_state).unwrap();
        let adf_state = element.get_child("AdfState").unwrap().clone().text.unwrap();
        let adf_state = AdfState::parse(&adf_state).unwrap();
        ScanStatus::new(scanner_state, adf_state)
    }
}

#[cfg(test)]
mod test {

    use super::*;

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
    fn read_scan_status_xml() {
        fn check_parse_scan_status(s: &str, scanner_state: ScannerState, adf_state: AdfState) {
            let status = s.as_bytes();
            let scan_status = ScanStatus::read_xml(status);
            assert_eq!(scanner_state, scan_status.get_scanner_state());
            assert_eq!(adf_state, scan_status.get_adf_state());
        }
        check_parse_scan_status(SCAN_STATUS_IDLE, ScannerState::Idle, AdfState::Empty);
        check_parse_scan_status(SCAN_STATUS_BUSY, ScannerState::BusyWithScanJob, AdfState::Empty);
        check_parse_scan_status(SCAN_STATUS_LOADED, ScannerState::Idle, AdfState::Loaded);
    }

}
