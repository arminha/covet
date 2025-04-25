#![forbid(unsafe_code)]

use anyhow::Result;
use clap::Parser;
use std::path::Path;
use tokio::runtime::Runtime;
use tracing::info;

mod cli;
mod jpeg;
mod message;
mod scanner;
mod util;
mod web;

use crate::cli::{Opt, ScanOpt, ScannerOpt};
use crate::message::scan_job::{ColorSpace, Format};
use crate::scanner::{Scanner, ScannerError};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let opt = Opt::parse();
    match opt {
        Opt::Status(opt) => {
            status(&opt)?;
        }
        Opt::Scan(opt) => {
            scan(&opt)?;
        }
        Opt::Web(opt) => {
            let use_tls = !opt.scanner_opts.no_tls;
            web::run_server(
                &opt.scanner_opts.scanner,
                &opt.listen,
                opt.port,
                use_tls,
                opt.disable_jpeg_fix,
            )?;
        }
        Opt::FixJpegHeight(opt) => {
            fix_jpeg_height(&opt.input, &opt.output)?;
        }
    }
    Ok(())
}

fn fix_jpeg_height(input: &Path, ouput: &Path) -> Result<()> {
    let input_buf = std::fs::read(input)?;
    if let Some(buffer) = jpeg::fix_jpeg_height(input_buf.into())? {
        std::fs::write(ouput, buffer)?;
    }
    Ok(())
}

fn status(opt: &ScannerOpt) -> Result<(), ScannerError> {
    let scanner = Scanner::new(&opt.scanner, !opt.no_tls, false);
    let rt = Runtime::new()?;
    rt.block_on(print_scan_status(&scanner))?;
    Ok(())
}

async fn print_scan_status(scanner: &Scanner) -> Result<(), ScannerError> {
    info!("Status of scanner {}", scanner.host());
    let status = scanner.get_scan_status().await?;
    info!(
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
    let scanner = Scanner::new(
        &opt.scanner_opts.scanner,
        !opt.scanner_opts.no_tls,
        opt.disable_jpeg_fix,
    );
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
