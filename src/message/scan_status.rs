use xmltree::Element;

use std::io::Read;
use std::str::FromStr;

use crate::message::error::ParseError;
use crate::message::util;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScannerState {
    Idle,
    BusyWithScanJob,
    /// For example a paper jam
    AdfError,
}

impl FromStr for ScannerState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<ScannerState, ParseError> {
        match s {
            "Idle" => Ok(ScannerState::Idle),
            "BusyWithScanJob" => Ok(ScannerState::BusyWithScanJob),
            "AdfError" => Ok(ScannerState::AdfError),
            _ => Err(ParseError::unknown_enum_value("ScannerState", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdfState {
    Empty,
    Loaded,
    PickFailure,
}

impl FromStr for AdfState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<AdfState, ParseError> {
        match s {
            "Empty" => Ok(AdfState::Empty),
            "Loaded" => Ok(AdfState::Loaded),
            "PickFailure" => Ok(AdfState::PickFailure),
            _ => Err(ParseError::unknown_enum_value("AdfState", s)),
        }
    }
}

#[derive(Debug)]
pub struct ScanStatus {
    scanner_state: ScannerState,
    adf_state: Option<AdfState>,
}

impl ScanStatus {
    pub fn new(scanner_state: ScannerState, adf_state: Option<AdfState>) -> ScanStatus {
        ScanStatus {
            scanner_state,
            adf_state,
        }
    }

    pub fn scanner_state(&self) -> ScannerState {
        self.scanner_state
    }

    pub fn is_idle(&self) -> bool {
        self.scanner_state == ScannerState::Idle
    }

    pub fn adf_state(&self) -> Option<AdfState> {
        self.adf_state
    }

    pub fn read_xml<R: Read>(r: R) -> Result<ScanStatus, ParseError> {
        let element = Element::parse(r)?;
        let scanner_state = util::parse_child_value(&element, "ScannerState")?;
        let adf_state = util::parse_optional_child_value(&element, "AdfState")?;
        Ok(ScanStatus::new(scanner_state, adf_state))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const SCAN_STATUS_IDLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
            <ScannerState>Idle</ScannerState>
            <AdfState>Empty</AdfState>
            </ScanStatus>"#;

    const SCAN_STATUS_BUSY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
            <ScannerState>BusyWithScanJob</ScannerState>
            <AdfState>Empty</AdfState>
            </ScanStatus>"#;

    const SCAN_STATUS_LOADED: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
            <ScannerState>Idle</ScannerState>
            <AdfState>Loaded</AdfState>
            </ScanStatus>"#;

    const SCAN_STATUS_ADF_ERROR: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
            <ScannerState>AdfError</ScannerState>
            <AdfState>PickFailure</AdfState>
            </ScanStatus>"#;

    const SCAN_STATUS_UNKNOWN_ADF_STATE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
            <ScannerState>Idle</ScannerState>
            <AdfState>Laoded</AdfState>
            </ScanStatus>"#;

    const SCAN_STATUS_NO_ADF: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
            <ScannerState>Idle</ScannerState>
            </ScanStatus>"#;

    #[test]
    fn read_scan_status_xml() {
        fn check_parse_scan_status(
            s: &str,
            scanner_state: ScannerState,
            adf_state: Option<AdfState>,
        ) {
            let status = s.as_bytes();
            let scan_status = ScanStatus::read_xml(status).expect("parsing failed");
            assert_eq!(scanner_state, scan_status.scanner_state());
            assert_eq!(adf_state, scan_status.adf_state());
        }
        check_parse_scan_status(SCAN_STATUS_IDLE, ScannerState::Idle, Some(AdfState::Empty));
        check_parse_scan_status(
            SCAN_STATUS_BUSY,
            ScannerState::BusyWithScanJob,
            Some(AdfState::Empty),
        );
        check_parse_scan_status(
            SCAN_STATUS_LOADED,
            ScannerState::Idle,
            Some(AdfState::Loaded),
        );
        check_parse_scan_status(SCAN_STATUS_NO_ADF, ScannerState::Idle, None);
        check_parse_scan_status(
            SCAN_STATUS_ADF_ERROR,
            ScannerState::AdfError,
            Some(AdfState::PickFailure),
        );
    }

    #[test]
    fn read_scan_status_xml_unknown_adf_state() {
        let error = ScanStatus::read_xml(SCAN_STATUS_UNKNOWN_ADF_STATE.as_bytes())
            .expect_err("parsing succeeded");
        assert_eq!(error.to_string(), "unknown AdfState: Laoded")
    }
}
