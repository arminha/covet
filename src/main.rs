#![forbid(unsafe_code)]

use anyhow::Result;
use env_logger;
use structopt::StructOpt;
use tokio::runtime::Runtime;

mod cli;
mod message;
mod scanner;
mod util;
mod web;

use crate::cli::{Opt, ScanOpt, ScannerOpt};
use crate::message::scan_job::{ColorSpace, Format};
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
        Opt::Web(opt) => {
            env_logger::init();
            let use_tls = !opt.scanner_opts.no_tls;
            web::run_server(&opt.scanner_opts.scanner, &opt.listen, opt.port, use_tls)?;
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
