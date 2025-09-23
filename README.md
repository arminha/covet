# covet

[![build](https://github.com/arminha/covet/workflows/build/badge.svg)](https://github.com/arminha/covet/actions?query=workflow%3Abuild)
[![dependency status](https://deps.rs/repo/github/arminha/covet/status.svg)](https://deps.rs/repo/github/arminha/covet)
[![Crates.io](https://img.shields.io/crates/v/covet)](https://crates.io/crates/covet)

A CLI and web frontend for scanning with HP Envy printer / scanners.

## Features

*   Supports HP Envy scanners
*   Scan documents from the command line or in a web UI
*   covet communicates with the scanner through a REST interface implemented in HP Envy scanners

## Installation

covet can be installed with cargo.

```
$ cargo install covet
```

## Usage

### Web UI

To start the web UI run
```
$ covet web <SCANNER>
```
where `<SCANNER>` is the hostname or IP address of your scanner. Then open [`http://127.0.0.1:3000/`](http://127.0.0.1:3000/) in your browser. It will show the following page with a simple form to scan a document.

![Web UI Screenshot](doc/screenshots/webui.png)

#### Options

```
$ covet web -h

Start a web server to handle scan jobs

Usage: covet web [OPTIONS] <SCANNER>

Arguments:
  <SCANNER>  The hostname of the scanner

Options:
      --no-tls            Do not use TLS to secure the connection to the scanner
  -p, --port <PORT>       Port to use for the web server [default: 3000]
  -l, --listen <ADDR>     Listen address to use for the web server [default: 127.0.0.1]
      --disable-jpeg-fix  Do not fix the heigt of JPEG files scanned from the automatic
                          document feeder
  -h, --help              Print help
```

### Command line scanning

To scan a document directly from the command line run
```
$ covet scan <SCANNER>
```
where `<SCANNER>` is the hostname or IP address of your scanner. This will scan a document and store it as a PDF in the current directory.

#### Options

```
$ covet scan -h

Scan a document

Usage: covet scan [OPTIONS] <SCANNER>

Arguments:
  <SCANNER>  The hostname of the scanner

Options:
      --no-tls
          Do not use TLS to secure the connection to the scanner
  -s, --source <SOURCE>
          The document source [default: auto] [possible values: auto, adf, glass]
  -f, --format <FORMAT>
          The format of the output [default: pdf] [possible values: pdf, jpeg]
  -c, --color <COLORSPACE>
          The color space of the output [default: color] [possible values: gray, color]
  -r, --resolution <RESOLUTION>
          The scan resolution in dpi [default: 300] [possible values: 300, 600]
  -q, --compression-quality <QUALITY>
          Compression quality level (lower is better) [default: 25]
      --disable-jpeg-fix
          Do not fix the heigt of JPEG files scanned from the automatic document feeder
  -h, --help
          Print help
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the terms of the GNU GENERAL PUBLIC LICENSE version 3 or later.
