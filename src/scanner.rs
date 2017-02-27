use hyper::client::{Client, Response};
use hyper::error;
use hyper::error::Result as HResult;
use hyper::header::Location;
use hyper::status::StatusCode;
use hyper::Url;

use time;

use std::fmt;
use std::fs::File;
use std::io;

use message::error::ParseError;
use message::job_status::ScanJobStatus;
use message::scan_job::{ScanJob, Format};
use message::scan_status::ScanStatus;

#[derive(Debug)]
pub enum ScannerError {
    Io(io::Error),
    Parse(ParseError),
    AdfEmpty,
    Busy,
    Other(String),
}

impl From<ParseError> for ScannerError {
    fn from(err: ParseError) -> Self {
        ScannerError::Parse(err)
    }
}

impl From<error::Error> for ScannerError {
    fn from(err: error::Error) -> Self {
        if let error::Error::Io(io) = err {
            ScannerError::Io(io)
        } else {
            ScannerError::Other(err.to_string())
        }
    }
}

impl From<String> for ScannerError {
    fn from(err: String) -> Self {
        ScannerError::Other(err)
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ScannerError::Io(ref err) => {
                write!(f, "{}", err)
            },
            &ScannerError::Parse(ref err) => {
                write!(f, "{}", err)
            },
            &ScannerError::AdfEmpty => {
                write!(f, "{}", "Adf is empty")
            },
            &ScannerError::Busy => {
                write!(f, "{}", "Scanner is busy")
            }
            &ScannerError::Other(ref err) => {
                write!(f, "{}", err)
            }
        }
    }
}

pub struct Scanner {
    host: String,
    client: Client,
}

impl Scanner {
    pub fn new(host: &str) -> Scanner {
        let client = Client::new();
        Scanner { host: host.to_owned(), client: client }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn get_scan_status(&self) -> Result<ScanStatus, ScannerError> {
        let resp = self.retrieve_scan_status()?;
        let status = ScanStatus::read_xml(resp)?;
        Ok(status)
    }

    fn retrieve_scan_status(&self) -> HResult<Response> {
        let url = format!("http://{}/Scan/Status", self.host);
        let url = Url::parse(&url)?;
        self.client.get(url).send()
    }

    pub fn start_job(&self, job: ScanJob) -> Result<String, ScannerError> {
        let mut target: Vec<u8> = Vec::new();
        job.write_xml(&mut target).unwrap();
        let result = String::from_utf8(target).unwrap();
        println!("{}", result);
        let url = format!("http://{}/Scan/Jobs", self.host);
        let url = Url::parse(&url).unwrap();

        let response = self.client.post(url).body(&result).send()?;
        if response.status != StatusCode::Created {
            return Err(ScannerError::Other(format!("Received status {}", response.status)));
        }
        let location: &Location = response.headers.get().unwrap();
        Ok(format!("{}", location))
    }

    pub fn get_job_status(&self, location: &str) -> Result<ScanJobStatus, ScannerError> {
        let url = Url::parse(location).map_err(|e| e.to_string())?;
        let response = self.client.get(url).send()?;
        let status = ScanJobStatus::read_xml(response)?;
        Ok(status)
    }

    pub fn download(&self, binary_url: &str, target: &str) -> Result<(), ScannerError> {
        let url = format!("http://{}{}", self.host, binary_url);
        let url = Url::parse(&url).map_err(|e| e.to_string())?;
        let mut response = self.client.get(url).send()?;
        let mut file = File::create(target).map_err(|e| e.to_string())?;
        io::copy(&mut response, &mut file).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn download_response(&self, binary_url: &str) -> Result<Response, ScannerError> {
        let url = format!("http://{}{}", self.host, binary_url);
        let url = Url::parse(&url).map_err(|e| e.to_string())?;
        let response = self.client.get(url).send()?;
        Ok(response)
    }
}

pub fn output_file_name(format: &Format, time: &time::Tm) -> String {
    let extension = match format {
        &Format::Pdf => "pdf",
        &Format::Jpeg => "jpeg"
    };
    let ts = time::strftime("%Y%m%d_%H%M%S", time).unwrap();
    format!("scan_{}.{}", ts, extension)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_output_file_name() {
        let time = time::at_utc(time::Timespec::new(1486905545, 0));
        assert_eq!("scan_20170212_131905.pdf", output_file_name(&Format::Pdf, &time));
        assert_eq!("scan_20170212_131905.jpeg", output_file_name(&Format::Jpeg, &time));
    }
}
