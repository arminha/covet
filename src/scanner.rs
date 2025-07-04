use bytes::{Bytes, BytesMut};
use futures_util::stream::{once, Stream};
use futures_util::StreamExt;
use jiff::Timestamp;
use reqwest::header::LOCATION;
use reqwest::{Client, Response, StatusCode, Url};
use thiserror::Error;
use tracing::{debug, error};

use std::io::{self, Cursor};

use crate::jpeg;
use crate::message::error::ParseError;
use crate::message::job_status::{PageState, ScanJobStatus};
use crate::message::scan_job::{Format, InputSource, ScanJob};
use crate::message::scan_status::ScanStatus;

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("Io error")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("Parse error")]
    Parse { source: ParseError, data: String },
    #[error("Adf is empty")]
    AdfEmpty,
    #[error("Scanner is busy")]
    Busy,
    #[error("Scanner is not available. Is it turned off?")]
    NotAvailable { source: reqwest::Error },
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    ReqwestError(reqwest::Error),
    #[error("Job creation failed: received status {0}")]
    JobCreationFailed(StatusCode),
    #[error("Job canceled")]
    Canceled,
}

impl ScannerError {
    fn form_parse_error(source: ParseError, data: Bytes) -> Self {
        let data = String::from_utf8_lossy(&data).into_owned();
        ScannerError::Parse { source, data }
    }
}

impl From<reqwest::Error> for ScannerError {
    fn from(err: reqwest::Error) -> Self {
        let not_available = err.is_connect();
        if not_available {
            ScannerError::NotAvailable { source: err }
        } else {
            ScannerError::ReqwestError(err)
        }
    }
}

#[derive(Debug)]
pub struct Scanner {
    client: Client,
    base_url: Url,
    disable_jpeg_fix: bool,
}

#[derive(Debug)]
pub struct Job<'a> {
    scanner: &'a Scanner,
    location: Url,
    binary_url: Option<String>,
    parameters: ScanJob,
}

impl Scanner {
    pub fn new(host: &str, use_tls: bool, disable_jpeg_fix: bool) -> Scanner {
        let client = Client::builder()
            .http1_title_case_headers()
            .build()
            .unwrap();
        let base_url_string = if use_tls {
            format!("https://{host}")
        } else {
            format!("http://{host}")
        };
        let base_url = base_url_string.parse().unwrap();
        Scanner {
            client,
            base_url,
            disable_jpeg_fix,
        }
    }

    pub fn host(&self) -> &str {
        self.base_url.host_str().unwrap()
    }

    pub async fn get_scan_status(&self) -> Result<ScanStatus, ScannerError> {
        let data = self.get("/Scan/Status").await?;
        let c = Cursor::new(&data);
        let status =
            ScanStatus::read_xml(c).map_err(|e| ScannerError::form_parse_error(e, data))?;
        Ok(status)
    }

    pub async fn start_job(&self, job: ScanJob) -> Result<Job<'_>, ScannerError> {
        let mut data: Vec<u8> = Vec::new();
        job.write_xml(&mut data).unwrap();
        let data: Bytes = data.into();
        let response = self.post("/Scan/Jobs", data).await?;
        let status = response.status();
        if status != StatusCode::CREATED {
            return Err(ScannerError::JobCreationFailed(status));
        }
        let location = response.headers().get(LOCATION).unwrap();
        let loc_url: Url = location.to_str().unwrap().parse()?;
        let loc_url_rebase = self.base_url.join(loc_url.path())?;
        debug!("job URL: {loc_url_rebase}");
        Ok(Job::new(self, loc_url_rebase, job))
    }

    async fn get(&self, path: &str) -> Result<Bytes, ScannerError> {
        let url = self.base_url.join(path)?;
        let data = self.client.get(url).send().await?.bytes().await?;
        Ok(data)
    }

    async fn post(&self, path: &str, data: Bytes) -> Result<Response, ScannerError> {
        let url = self.base_url.join(path)?;
        let response = self.client.post(url).body(data).send().await?;
        Ok(response)
    }

    async fn get_job_status(&self, job: &Job<'_>) -> Result<ScanJobStatus, ScannerError> {
        let url = job.location.clone();
        let data = self.client.get(url).send().await?.bytes().await?;
        let c = Cursor::new(&data);
        let status =
            ScanJobStatus::read_xml(c).map_err(|e| ScannerError::form_parse_error(e, data))?;
        Ok(status)
    }

    async fn download_stream(
        &self,
        binary_url: &str,
    ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, ScannerError> {
        let url = self.base_url.join(binary_url)?;
        let response = self.client.get(url).send().await?;
        Ok(response.bytes_stream())
    }
}

impl Job<'_> {
    fn new(scanner: &Scanner, location: Url, parameters: ScanJob) -> Job<'_> {
        Job {
            scanner,
            location,
            binary_url: None,
            parameters,
        }
    }

    pub async fn retrieve_status(&mut self) -> Result<bool, ScannerError> {
        // TODO error handling
        let status = self.scanner.get_job_status(self).await?;
        let page = status.pages().first().unwrap();
        let page_state = page.state();
        match page_state {
            PageState::ReadyToUpload { binary_url } => {
                self.binary_url = Some(binary_url.clone());
                Ok(true)
            }
            PageState::CanceledByDevice => Err(ScannerError::Canceled),
            _ => Ok(false),
        }
    }

    pub async fn download_stream(
        self,
    ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, ScannerError> {
        // TODO error handling
        let mut stream = self
            .scanner
            .download_stream(&self.binary_url.unwrap())
            .await?;
        if !self.scanner.disable_jpeg_fix
            && self.parameters.input_source == InputSource::Adf
            && self.parameters.format == Format::Jpeg
        {
            let mut data = read_all_bytes_from_stream(&mut stream).await?;
            match jpeg::fix_jpeg_height(data.clone()) {
                Ok(Some(buffer)) => data = buffer,
                Ok(None) => (),
                Err(e) => error!("Cannot fix jpeg headers. {e}"),
            };
            return Ok(bytes_into_stream(data).boxed());
        }
        Ok(stream.boxed())
    }
}

async fn read_all_bytes_from_stream(
    mut stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Unpin,
) -> Result<Bytes, ScannerError> {
    let mut data = BytesMut::new();
    while let Some(item) = stream.next().await {
        data.extend_from_slice(item?.as_ref());
    }
    let data = data.freeze();
    Ok(data)
}

fn bytes_into_stream(data: Bytes) -> impl Stream<Item = Result<Bytes, reqwest::Error>> {
    once(async { Ok(data) })
}

pub fn output_file_name(format: Format, time: &Timestamp) -> String {
    let extension = match format {
        Format::Pdf => "pdf",
        Format::Jpeg => "jpeg",
    };
    let ts = time.strftime("%Y%m%d_%H%M%S");
    format!("scan_{ts}.{extension}")
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn check_output_file_name() {
        let time = Timestamp::from_second(1486905545).unwrap();
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
