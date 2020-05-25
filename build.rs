use std::env;
use structopt::clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = Opt::clap();
    app.gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, outdir);
}
