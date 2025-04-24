#![forbid(unsafe_code)]

use anyhow::Result;
use bytes::Bytes;
use clap::Parser;
use std::path::Path;
use tokio::runtime::Runtime;

mod cli;
mod jpeg;
mod message;
mod scanner;
mod util;
mod web;

use crate::cli::{Opt, ScanOpt, ScannerOpt};
use crate::jpeg::Jpeg;
use crate::message::scan_job::{ColorSpace, Format};
use crate::scanner::{Scanner, ScannerError};

fn main() -> Result<()> {
    let opt = Opt::parse();
    match opt {
        Opt::Status(opt) => {
            status(&opt)?;
        }
        Opt::Scan(opt) => {
            scan(&opt)?;
        }
        Opt::Web(opt) => {
            env_logger::init();
            let use_tls = !opt.scanner_opts.no_tls;
            web::run_server(&opt.scanner_opts.scanner, &opt.listen, opt.port, use_tls)?;
        }
        Opt::FixJpegHeight(opt) => {
            fix_jpeg_height(&opt.input, &opt.output)?;
        }
    }
    Ok(())
}

fn fix_jpeg_height(input: &Path, ouput: &Path) -> Result<()> {
    let input_buf = std::fs::read(input)?;
    let jpeg = Jpeg::from_bytes(input_buf.into())?;

    println!("{jpeg}");

    if let Some(height) = jpeg.get_height_from_dnl() {
        let jpeg = jpeg.with_height(height);
        let buffer: Bytes = jpeg.into();
        std::fs::write(ouput, buffer)?;
    }
    Ok(())
}

fn status(opt: &ScannerOpt) -> Result<(), ScannerError> {
    let scanner = Scanner::new(&opt.scanner, !opt.no_tls);
    let rt = Runtime::new()?;
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
            cli::Format::Pdf => Format::Pdf,
            cli::Format::Jpeg => Format::Jpeg,
        }
    }
}

impl cli::ColorSpace {
    fn to_internal(self) -> ColorSpace {
        match self {
            cli::ColorSpace::Gray => ColorSpace::Gray,
            cli::ColorSpace::Color => ColorSpace::Color,
        }
    }
}

fn scan(opt: &ScanOpt) -> Result<(), ScannerError> {
    let scanner = Scanner::new(&opt.scanner_opts.scanner, !opt.scanner_opts.no_tls);
    let rt = Runtime::new()?;
    rt.block_on(util::scan_to_file(
        scanner,
        opt.format.to_internal(),
        opt.color.to_internal(),
        opt.source,
        opt.resolution,
        opt.compression_quality,
    ))?;
    Ok(())
}
