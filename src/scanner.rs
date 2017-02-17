extern crate hyper;

use self::hyper::client::{Client, Response};
use self::hyper::error::Result as HResult;
use self::hyper::header::Location;
use self::hyper::status::StatusCode;
use self::hyper::Url;

use std::fs::File;
use std::io;

use job_status::ScanJobStatus;
use scan_job::ScanJob;
use scan_status::ScanStatus;

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

    pub fn get_scan_status(&self) -> Result<ScanStatus, String> {
        self.retrieve_scan_status()
            .map_err(|e| e.to_string())
            .and_then(|r| ScanStatus::read_xml(r).map_err(|e| e.to_string()))
    }

    fn retrieve_scan_status(&self) -> HResult<Response> {
        let url = format!("http://{}/Scan/Status", self.host);
        let url = Url::parse(&url)?;
        self.client.get(url).send()
    }

    pub fn start_job(&self, job: ScanJob) -> Result<String, String> {
        let mut target: Vec<u8> = Vec::new();
        job.write_xml(&mut target).unwrap();
        let result = String::from_utf8(target).unwrap();
        println!("{}", result);
        let url = format!("http://{}/Scan/Jobs", self.host);
        let url = Url::parse(&url).unwrap();

        let response = self.client.post(url).body(&result).send().map_err(|e| e.to_string())?;
        if response.status != StatusCode::Created {
            return Err(format!("Received status {}", response.status));
        }
        let location: &Location = response.headers.get().unwrap();
        Ok(format!("{}", location))
    }

    pub fn get_job_status(&self, location: &str) -> Result<ScanJobStatus, String> {
        let url = Url::parse(location).map_err(|e| e.to_string())?;
        self.client.get(url).send()
            .map_err(|e| e.to_string())
            .and_then(|r| ScanJobStatus::read_xml(r).map_err(|e| e.to_string()))
    }

    pub fn download(&self, binary_url: &str, target: &str) -> Result<(), String> {
        let url = format!("http://{}{}", self.host, binary_url);
        let url = Url::parse(&url).map_err(|e| e.to_string())?;
        let mut response = self.client.get(url).send().map_err(|e| e.to_string())?;
        let mut file = File::create(target).map_err(|e| e.to_string())?;
        io::copy(&mut response, &mut file).map_err(|e| e.to_string())?;
        Ok(())
    }
}
