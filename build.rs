use clap::{CommandFactory, Error};
use clap_complete::{generate_to, shells::Bash};
use std::env;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Opt::command();
    let path = generate_to(Bash, &mut cmd, env!("CARGO_PKG_NAME"), outdir)?;

    println!("cargo:info=completion file is generated: {path:?}");
    Ok(())
}
