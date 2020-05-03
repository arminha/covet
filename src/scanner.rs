/*
Copyright (C) 2019  Armin Häberling

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use hyper::client::{Client, Response};
use hyper::error::Result as HResult;
use hyper::header::Location;
use hyper::net::HttpsConnector;
use hyper::status::StatusCode;
use hyper::Url;
use hyper_native_tls::NativeTlsClient;

use thiserror::Error;
use time::OffsetDateTime;

use std::fs::File;
use std::io::{self, ErrorKind, Read};

use crate::message::error::ParseError;
use crate::message::job_status::{PageState, ScanJobStatus};
use crate::message::scan_job::{Format, ScanJob};
use crate::message::scan_status::ScanStatus;

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error(transparent)]
    Io(io::Error),
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error("Adf is empty")]
    AdfEmpty,
    #[error("Scanner is busy")]
    Busy,
    #[error("Scanner is not available. Is it turned off? Cause: {0}")]
    NotAvailable(io::Error),
    #[error(transparent)]
    UrlParseError(#[from] hyper::error::ParseError),
    #[error(transparent)]
    HyperError(hyper::error::Error),
    #[error("Received status {0}")]
    JobCreationFailed(StatusCode),
}

impl From<hyper::error::Error> for ScannerError {
    fn from(err: hyper::error::Error) -> Self {
        if let hyper::error::Error::Io(io) = err {
            ScannerError::from(io)
        } else {
            ScannerError::HyperError(err)
        }
    }
}

impl From<io::Error> for ScannerError {
    fn from(err: io::Error) -> Self {
        let not_available = match err.kind() {
            ErrorKind::ConnectionRefused => true,
            ErrorKind::Other if format!("{}", err).contains("Name or service not known") => true,
            _ => match err.raw_os_error() {
                // ECONNREFUSED - 111 - Connection refused or EHOSTUNREACH 113 No route to host
                Some(111) | Some(113) => true,
                _ => false,
            },
        };
        if not_available {
            ScannerError::NotAvailable(err)
        } else {
            ScannerError::Io(err)
        }
    }
}

#[derive(Debug)]
pub struct Scanner {
    base_url: Url,
    client: Client,
}

#[derive(Debug)]
pub struct Job<'a> {
    scanner: &'a Scanner,
    location: Url,
    binary_url: Option<String>,
}

impl Scanner {
    pub fn new(host: &str, use_tls: bool) -> Scanner {
        let client = if use_tls {
            let ssl = NativeTlsClient::new().unwrap();
            let connector = HttpsConnector::new(ssl);
            Client::with_connector(connector)
        } else {
            Client::new()
        };
        let base_url_string = if use_tls {
            format!("https://{}", host)
        } else {
            format!("http://{}", host)
        };
        let base_url = Url::parse(&base_url_string).unwrap();
        Scanner { client, base_url }
    }

    pub fn host(&self) -> &str {
        self.base_url.host_str().unwrap()
    }

    pub fn get_scan_status(&self) -> Result<ScanStatus, ScannerError> {
        let resp = self.retrieve_scan_status()?;
        let status = ScanStatus::read_xml(resp)?;
        Ok(status)
    }

    fn retrieve_scan_status(&self) -> HResult<Response> {
        let url = self.base_url.join("/Scan/Status")?;
        self.client.get(url).send()
    }

    pub fn start_job(&self, job: ScanJob) -> Result<Job<'_>, ScannerError> {
        let mut target: Vec<u8> = Vec::new();
        job.write_xml(&mut target).unwrap();
        let result = String::from_utf8(target).unwrap();
        println!("{}", result);
        let url = self.base_url.join("/Scan/Jobs")?;
        let response = self.client.post(url).body(&result).send()?;
        if response.status != StatusCode::Created {
            return Err(ScannerError::JobCreationFailed(response.status));
        }
        let location: &Location = response.headers.get().unwrap();
        let loc_url = Url::parse(location)?;
        let loc_url_rebase = self.base_url.join(loc_url.path())?;
        println!("{}", loc_url_rebase);
        Ok(Job::new(self, loc_url_rebase))
    }

    fn get_job_status(&self, job: &Job<'_>) -> Result<ScanJobStatus, ScannerError> {
        let response = self.client.get(job.location.clone()).send()?;
        let status = ScanJobStatus::read_xml(response)?;
        Ok(status)
    }

    fn download_reader(&self, binary_url: &str) -> Result<Box<dyn Read + Send>, ScannerError> {
        let url = self.base_url.join(binary_url)?;
        let response = self.client.get(url).send()?;
        Ok(Box::new(response))
    }
}

impl<'a> Job<'a> {
    fn new(scanner: &Scanner, location: Url) -> Job<'_> {
        Job {
            scanner,
            location,
            binary_url: None,
        }
    }

    pub fn retrieve_status(&mut self) -> Result<bool, ScannerError> {
        // TODO error handling
        let status = self.scanner.get_job_status(self)?;
        let page = status.pages().get(0).unwrap();
        let page_state = page.state();
        if page_state == PageState::ReadyToUpload {
            self.binary_url = Some(page.binary_url().unwrap().to_owned());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn download_reader(self) -> Result<Box<dyn Read + Send>, ScannerError> {
        // TODO error handling
        self.scanner.download_reader(&self.binary_url.unwrap())
    }

    pub fn download_to_file(self, target: &str) -> Result<(), ScannerError> {
        let mut reader = self.download_reader()?;
        let mut file = File::create(target)?;
        io::copy(&mut reader, &mut file)?;
        Ok(())
    }
}

pub fn output_file_name(format: Format, time: &OffsetDateTime) -> String {
    let extension = match format {
        Format::Pdf => "pdf",
        Format::Jpeg => "jpeg",
    };
    let ts = time.format("%Y%m%d_%H%M%S");
    format!("scan_{}.{}", ts, extension)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_output_file_name() {
        let time = OffsetDateTime::from_unix_timestamp(1486905545);
        assert_eq!(
            "scan_20170212_131905.pdf",
            output_file_name(Format::Pdf, &time)
        );
        assert_eq!(
            "scan_20170212_131905.jpeg",
            output_file_name(Format::Jpeg, &time)
        );
    }
}
