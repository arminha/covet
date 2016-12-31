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
}
