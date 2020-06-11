use bytes::Bytes;
use futures_util::stream::{Stream, StreamExt};
use time::OffsetDateTime;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use std::time::Duration;

use crate::cli::Source;
use crate::message::scan_job::{ColorSpace, Format, InputSource, ScanJob};
use crate::message::scan_status::AdfState;
use crate::scanner::{self, Scanner, ScannerError};

pub(crate) async fn scan_to_file(
    scanner: Scanner,
    format: Format,
    color: ColorSpace,
    source: Source,
    resolution: u32,
    quality: u32,
) -> Result<(), ScannerError> {
    let mut stream = scan_to_stream(&scanner, format, color, source, resolution, quality).await?;
    let file_name = scanner::output_file_name(format, &OffsetDateTime::now_utc());
    let mut file = File::create(file_name).await?;
    while let Some(item) = stream.next().await {
        file.write_all(item?.as_ref()).await?;
    }
    Ok(())
}

pub(crate) async fn scan_to_stream(
    scanner: &Scanner,
    format: Format,
    color: ColorSpace,
    source: Source,
    resolution: u32,
    quality: u32,
) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, ScannerError> {
    let status = scanner.get_scan_status().await?;
    if !status.is_idle() {
        return Err(ScannerError::Busy);
    }
    let input_source = choose_source(source, status.adf_state())?;
    let mut job = scanner
        .start_job(ScanJob::new(
            input_source,
            resolution,
            quality,
            format,
            color,
        ))
        .await?;
    println!("Job: {:?}", job);
    loop {
        let ready = job.retrieve_status().await?;
        if ready {
            println!("Job: {:?}", job);
            return job.download_stream().await;
        }
        tokio::time::delay_for(Duration::from_millis(500)).await;
    }
}

pub(crate) fn choose_source(
    source: Source,
    adf_state: Option<AdfState>,
) -> Result<InputSource, ScannerError> {
    match adf_state {
        Some(AdfState::Loaded) => match source {
            Source::adf | Source::auto => Ok(InputSource::Adf),
            Source::glass => Ok(InputSource::Platen),
        },
        _ => match source {
            Source::adf => Err(ScannerError::AdfEmpty),
            _ => Ok(InputSource::Platen),
        },
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_choose_source() {
        assert_eq!(
            choose_source(Source::glass, None).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::glass, Some(AdfState::Empty)).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::glass, Some(AdfState::Loaded)).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::auto, None).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::auto, Some(AdfState::Empty)).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::auto, Some(AdfState::Loaded)).unwrap(),
            InputSource::Adf
        );
        assert!(choose_source(Source::adf, None).is_err());
        assert!(choose_source(Source::adf, Some(AdfState::Empty)).is_err());
        assert_eq!(
            choose_source(Source::adf, Some(AdfState::Loaded)).unwrap(),
            InputSource::Adf
        );
    }
}
