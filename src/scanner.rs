extern crate hyper;

use self::hyper::Url;
use self::hyper::client::{Client, Response};
use self::hyper::error::Result as HResult;
use self::hyper::header::Location;
use self::hyper::status::StatusCode;

use scan_status::ScanStatus;
use scan_job::ScanJob;

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
            .and_then(ScanStatus::read_xml)
    }

    fn retrieve_scan_status(&self) -> HResult<Response> {
        let url = format!("http://{}/Scan/Status", self.host);
        let url = try!(Url::parse(&url));
        self.client.get(url).send()
    }

    pub fn start_job(&self, job: ScanJob) -> Result<String, String> {
        let mut target: Vec<u8> = Vec::new();
        job.write_xml(&mut target).unwrap();
        let result = String::from_utf8(target).unwrap();
        println!("{}", result);
        let url = format!("http://{}/Scan/Jobs", self.host);
        let url = Url::parse(&url).unwrap();

        let response = match self.client.post(url).body(&result).send() {
            Ok(r) => r,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        if response.status != StatusCode::Created {
            return Err(format!("Received status {}", response.status));
        }
        println!("{:?}", response);
        let location: &Location = response.headers.get().unwrap();
        Ok(format!("{}", location))
    }
}
