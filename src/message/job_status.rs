use xmltree::Element;

use std::io::Read;
use std::convert::From;
use std::num::ParseIntError;
use std::str::FromStr;

use message::error::ParseError;
use message::util;

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::new(err.to_string())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JobState {
    Processing,
    Completed,
    Canceled,
}

impl FromStr for JobState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<JobState, ParseError> {
        match s {
            "Processing" => Ok(JobState::Processing),
            "Completed" => Ok(JobState::Completed),
            "Canceled" => Ok(JobState::Canceled),
            _ => Err(ParseError::new(format!("Unknown JobState: {}", s))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PageState {
    PreparingScan,
    ReadyToUpload,
    UploadCompleted,
}

impl FromStr for PageState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<PageState, ParseError> {
        match s {
            "PreparingScan" => Ok(PageState::PreparingScan),
            "ReadyToUpload" => Ok(PageState::ReadyToUpload),
            "UploadCompleted" => Ok(PageState::UploadCompleted),
            _ => Err(ParseError::new(format!("Unknown PageState: {}", s))),
        }
    }
}

#[derive(Debug)]
pub struct ScanPage {
    number: u32,
    state: PageState,
    binary_url: Option<String>,
}

impl ScanPage {
    pub fn new(number: u32, state: PageState, binary_url: Option<String>) -> ScanPage {
        ScanPage {
            number: number,
            state: state,
            binary_url: binary_url,
        }
    }

    pub fn state(&self) -> PageState {
        self.state
    }

    pub fn binary_url(&self) -> Option<&String> {
        self.binary_url.as_ref()
    }
}

#[derive(Debug)]
pub struct ScanJobStatus {
    state: JobState,
    pages: Vec<ScanPage>,
}

fn read_page(element: &Element) -> Result<ScanPage, ParseError> {
    let number: u32 = util::read_child_value(element, "PageNumber")?.parse()?;
    let state: PageState = util::parse_child_value(element, "PageState")?;
    let url = util::read_child_value(element, "BinaryURL").ok().map(|v| {
        v.to_string()
    });
    Ok(ScanPage::new(number, state, url))
}

impl ScanJobStatus {
    pub fn new(state: JobState, pages: Vec<ScanPage>) -> ScanJobStatus {
        ScanJobStatus {
            state: state,
            pages: pages,
        }
    }

    pub fn pages(&self) -> &Vec<ScanPage> {
        &self.pages
    }

    pub fn read_xml<R: Read>(r: R) -> Result<ScanJobStatus, ParseError> {
        let element = Element::parse(r)?;
        let state = util::parse_child_value(&element, "JobState")?;
        let job = element.get_child("ScanJob").ok_or("missing ScanJob")?;
        let mut pages = Vec::new();
        for child in &job.children {
            match child.name.as_ref() {
                "PreScanPage" | "PostScanPage" => {
                    pages.push(read_page(child)?);
                }
                _ => (),
            }
        }

        Ok(ScanJobStatus::new(state, pages))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const FULL_JOB_STATUS: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
            <j:Job xmlns:j=\"http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30\">\
            <j:JobUrl>/Jobs/JobList/2</j:JobUrl>\
            <j:JobCategory>Scan</j:JobCategory>\
            <j:JobState>Processing</j:JobState>\
            <j:JobStateUpdate>42-6</j:JobStateUpdate>\
            <ScanJob xmlns=\"http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19\">\
            <PreScanPage>\
            <PageNumber>1</PageNumber>\
            <PageState>PreparingScan</PageState>\
            <BufferInfo>\
            <ScanSettings>\
            <XResolution>300</XResolution>\
            <YResolution>300</YResolution>\
            <XStart>0</XStart>\
            <YStart>0</YStart>\
            <Width>2480</Width>\
            <Height>3508</Height>\
            <Format>Jpeg</Format>\
            <CompressionQFactor>25</CompressionQFactor>\
            <ColorSpace>Color</ColorSpace>\
            <BitDepth>8</BitDepth>\
            <InputSource>Platen</InputSource>\
            <ContentType>Photo</ContentType>\
            </ScanSettings>\
            <ImageWidth>2480</ImageWidth>\
            <ImageHeight>3508</ImageHeight>\
            <BytesPerLine>7440</BytesPerLine>\
            <Cooked>enabled</Cooked>\
            </BufferInfo>\
            <BinaryURL>/Scan/Jobs/2/Pages/1</BinaryURL>\
            <ImageOrientation>Normal</ImageOrientation>\
            </PreScanPage>\
            </ScanJob>\
            </j:Job>";

    const READY_TO_UPLOAD: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
            <j:Job xmlns:j=\"http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30\">\
            <j:JobUrl>/Jobs/JobList/4</j:JobUrl>\
            <j:JobCategory>Scan</j:JobCategory>\
            <j:JobState>Processing</j:JobState>\
            <j:JobStateUpdate>42-6</j:JobStateUpdate>\
            <ScanJob xmlns=\"http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19\">
            <PreScanPage>\
            <PageNumber>1</PageNumber>\
            <PageState>ReadyToUpload</PageState>\
            <BinaryURL>/Scan/Jobs/4/Pages/1</BinaryURL>\
            </PreScanPage>\
            </ScanJob>\
            </j:Job>";

    const COMPLETED: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
            <j:Job xmlns:j=\"http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30\">\
            <j:JobUrl>/Jobs/JobList/6</j:JobUrl>
            <j:JobCategory>Scan</j:JobCategory>
            <j:JobState>Completed</j:JobState>
            <j:JobStateUpdate>42-23</j:JobStateUpdate>
            <j:JobSource>userIO</j:JobSource>
            <ScanJob xmlns=\"http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19\">\
            <PostScanPage>\
            <PageNumber>2</PageNumber>\
            <PageState>UploadCompleted</PageState>\
            <TotalLines>3501</TotalLines>\
            </PostScanPage>\
            </ScanJob>\
            </j:Job>";

    fn parse_job_status(s: &str) -> ScanJobStatus {
        let status = s.as_bytes();
        ScanJobStatus::read_xml(status).expect("parsing failed")
    }

    fn check_one_page(job_status: &ScanJobStatus, num: u32, ps: PageState, bin_url: Option<&str>) {
        assert_eq!(1, job_status.pages().len());
        let page = job_status.pages().get(0).unwrap();
        assert_eq!(num, page.number);
        assert_eq!(ps, page.state());
        assert_eq!(bin_url.map(|v| v.to_string()).as_ref(), page.binary_url())
    }

    #[test]
    fn read_job_status_xml_preparing() {
        let status = parse_job_status(FULL_JOB_STATUS);
        assert_eq!(JobState::Processing, status.state);
        check_one_page(
            &status,
            1,
            PageState::PreparingScan,
            Some("/Scan/Jobs/2/Pages/1"),
        );
    }

    #[test]
    fn read_job_status_xml_ready_to_upload() {
        let status = parse_job_status(READY_TO_UPLOAD);
        assert_eq!(JobState::Processing, status.state);
        check_one_page(
            &status,
            1,
            PageState::ReadyToUpload,
            Some("/Scan/Jobs/4/Pages/1"),
        );
    }

    #[test]
    fn read_job_status_xml_completed() {
        let status = parse_job_status(COMPLETED);
        assert_eq!(JobState::Completed, status.state);
        check_one_page(&status, 2, PageState::UploadCompleted, None);
    }

}
