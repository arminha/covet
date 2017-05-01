use xmltree::Element;

use std::io::Read;

use message::error::ParseError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScannerState {
    Idle,
    BusyWithScanJob,
}

impl ScannerState {
    pub fn parse(s: &str) -> Result<ScannerState, ParseError> {
        match s {
            "Idle" => Ok(ScannerState::Idle),
            "BusyWithScanJob" => Ok(ScannerState::BusyWithScanJob),
            _ => Err(ParseError::new(format!("Unknown ScannerState: {}", s))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AdfState {
    Empty,
    Loaded,
}

impl AdfState {
    pub fn parse(s: &str) -> Result<AdfState, ParseError> {
        match s {
            "Empty" => Ok(AdfState::Empty),
            "Loaded" => Ok(AdfState::Loaded),
            _ => Err(ParseError::new(format!("Unknown AdfState: {}", s))),
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
        ScanStatus {
            scanner_state: scanner_state,
            adf_state: adf_state,
        }
    }

    pub fn scanner_state(&self) -> ScannerState {
        self.scanner_state
    }

    pub fn is_idle(&self) -> bool {
        self.scanner_state == ScannerState::Idle
    }

    pub fn adf_state(&self) -> AdfState {
        self.adf_state
    }

    pub fn read_xml<R: Read>(r: R) -> Result<ScanStatus, ParseError> {
        let element = Element::parse(r)?;
        let scanner_state = element
            .get_child("ScannerState")
            .and_then(|v| v.clone().text)
            .ok_or(ParseError::new("missing ScannerState"))
            .and_then(|v| ScannerState::parse(&v))?;
        let adf_state = element
            .get_child("AdfState")
            .and_then(|v| v.clone().text)
            .ok_or(ParseError::new("missing AdfState"))
            .and_then(|v| AdfState::parse(&v))?;
        Ok(ScanStatus::new(scanner_state, adf_state))
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
            let scan_status = ScanStatus::read_xml(status).expect("parsing failed");
            assert_eq!(scanner_state, scan_status.scanner_state());
            assert_eq!(adf_state, scan_status.adf_state());
        }
        check_parse_scan_status(SCAN_STATUS_IDLE, ScannerState::Idle, AdfState::Empty);
        check_parse_scan_status(SCAN_STATUS_BUSY,
                                ScannerState::BusyWithScanJob,
                                AdfState::Empty);
        check_parse_scan_status(SCAN_STATUS_LOADED, ScannerState::Idle, AdfState::Loaded);
    }

}
