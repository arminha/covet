#![forbid(unsafe_code)]

use anyhow::Result;
use structopt::StructOpt;
use time::OffsetDateTime;
use tokio::runtime::Runtime;

mod cli;
mod message;
mod scanner;
mod util;
// mod web;

use std::time::Duration;

use crate::cli::{Opt, ScanOpt, ScannerOpt};
use crate::message::scan_job::{ColorSpace, Format, ScanJob};
use crate::scanner::{Scanner, ScannerError};

fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Status(opt) => {
            status(opt)?;
        }
        Opt::Scan(opt) => {
            scan(opt)?;
        }
        Opt::Web(_opt) => {
            //let use_tls = !opt.scanner_opts.no_tls;
            //web::run_server(&opt.scanner_opts.scanner, &opt.listen, opt.port, use_tls)?;
        }
    }
    Ok(())
}

fn status(opt: ScannerOpt) -> Result<(), ScannerError> {
    let scanner = Scanner::new(&opt.scanner, !opt.no_tls);
    let mut rt = Runtime::new()?;
    rt.block_on(print_scan_status(&scanner))?;
    Ok(())
}

async fn print_scan_status(scanner: &Scanner) -> Result<(), ScannerError> {
    println!("Status of scanner {}", scanner.host());
    let status = scanner.get_scan_status().await?;
    println!(
        "Scanner: {:?}, Adf: {:?}",
        status.scanner_state(),
        status.adf_state()
    );
    Ok(())
}

impl cli::Format {
    fn to_internal(self) -> Format {
        match self {
            cli::Format::pdf => Format::Pdf,
            cli::Format::jpeg => Format::Jpeg,
        }
    }
}

impl cli::ColorSpace {
    fn to_internal(self) -> ColorSpace {
        match self {
            cli::ColorSpace::gray => ColorSpace::Gray,
            cli::ColorSpace::color => ColorSpace::Color,
        }
    }
}

fn scan(opt: ScanOpt) -> Result<(), ScannerError> {
    let scanner = Scanner::new(&opt.scanner_opts.scanner, !opt.scanner_opts.no_tls);
    let mut rt = Runtime::new()?;
    rt.block_on(scan_async(
        scanner,
        opt.format.to_internal(),
        opt.color.to_internal(),
        opt.source,
        opt.resolution,
        opt.compression_quality,
    ))?;
    Ok(())
}

async fn scan_async(
    scanner: Scanner,
    format: Format,
    color: ColorSpace,
    source: cli::Source,
    resolution: u32,
    quality: u32,
) -> Result<(), ScannerError> {
    let status = scanner.get_scan_status().await?;
    if !status.is_idle() {
        return Err(ScannerError::Busy);
    }
    let input_source = util::choose_source(source, status.adf_state())?;
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
            let output_file = scanner::output_file_name(format, &OffsetDateTime::now_utc());
            job.download_to_file(&output_file).await?;
            break;
        }
        tokio::time::delay_for(Duration::from_millis(500)).await;
    }
    Ok(())
}
