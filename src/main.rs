mod walker;

use std::path::PathBuf;

use clap::Parser;
use ignore::WalkBuilder;

use crate::walker::MyWalkerBuilder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// HTTP server base url
    base_url: String,

    /// Root directory
    root_dir: PathBuf,

    /// Thread count
    #[arg(short, default_value_t = 8)]
    threads: usize,

    /// File hash
    #[arg(short, default_value_t = false)]
    file_hash: bool,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    let walk = WalkBuilder::new(&cli.root_dir)
        .git_ignore(false)
        .threads(cli.threads)
        .build_parallel();
    walk.visit(&mut MyWalkerBuilder::new(&cli));
}
