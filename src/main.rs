mod walker;

use std::path::PathBuf;

use clap::Parser;
use ignore::WalkBuilder;

use crate::walker::MyWalkerBuilder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// HTTP server base url
    base_url: String,

    /// Root directory
    root_dir: PathBuf,

    /// Thread count
    #[arg(short, default_value_t = 8)]
    threads: usize,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let cli = Cli::parse();
    let walk = WalkBuilder::new(&cli.root_dir)
        .git_ignore(false)
        .threads(cli.threads)
        .build_parallel();
    walk.visit(&mut MyWalkerBuilder::new(
        &cli.base_url,
        &cli.root_dir.as_path().display().to_string(),
    ));
    Ok(())
}
