use clap::builder::TypedValueParser as _;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Source {
    Auto,
    Adf,
    Glass,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Format {
    Pdf,
    Jpeg,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ColorSpace {
    Gray,
    Color,
}

#[derive(Parser, Debug)]
pub struct ScannerOpt {
    /// The hostname of the scanner
    #[arg(name = "SCANNER")]
    pub scanner: String,

    /// Do not use TLS to secure the connection to the scanner
    #[arg(long)]
    pub no_tls: bool,
}

#[derive(Parser, Debug)]
pub struct ScanOpt {
    #[clap(flatten)]
    pub scanner_opts: ScannerOpt,

    /// The document source
    #[arg(
        short,
        long,
        name = "SOURCE",
        default_value = "auto",
        ignore_case(true)
    )]
    pub source: Source,

    /// The format of the output
    #[arg(short, long, name = "FORMAT", default_value = "pdf", ignore_case(true))]
    pub format: Format,

    /// The color space of the output
    #[arg(
        short,
        long,
        name = "COLORSPACE",
        default_value = "color",
        ignore_case(true)
    )]
    pub color: ColorSpace,

    /// The scan resolution in dpi
    #[arg(
        short,
        long,
        name = "RESOLUTION",
        default_value_t = 300,
        value_parser = clap::builder::PossibleValuesParser::new(["300", "600"])
            .map(|s| s.parse::<u32>().unwrap()),
    )]
    pub resolution: u32,

    /// Compression quality level (lower is better)
    #[arg(
        short = 'q',
        long = "compression-quality",
        name = "QUALITY",
        default_value = "25"
    )]
    pub compression_quality: u32,
}

#[derive(Parser, Debug)]
pub struct WebOpt {
    #[clap(flatten)]
    pub scanner_opts: ScannerOpt,

    /// Port to use for the web server
    #[arg(short, long, name = "PORT", default_value = "3000")]
    pub port: u16,

    /// Listen address to use for the web server
    #[arg(short, long, name = "ADDR", default_value = "127.0.0.1")]
    pub listen: String,
}

#[derive(Parser, Debug)]
#[clap(version)]
pub enum Opt {
    /// Display the status of the scanner
    Status(ScannerOpt),

    /// Scan a document
    Scan(ScanOpt),

    /// Start a web server to handle scan jobs
    Web(WebOpt),
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Opt::command().debug_assert()
}
