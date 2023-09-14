mod walker;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use ignore::WalkBuilder;

use crate::walker::{MyWalkerBuilder, process};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// HTTP server base url
    base_url: String,

    /// Root directory
    root_dir: PathBuf,

    /// Thread count
    #[arg(short, default_value_t = num_cpus::get() - 2)]
    threads: usize,

    /// File hash
    #[arg(short, default_value_t = false)]
    file_hash: bool,

    /// Ignore files with .gitignore
    #[arg(short, default_value_t = false)]
    git_ignore: bool,

    /// Sequential file walking or not
    #[arg(short, default_value_t = false)]
    sequential: bool,
}

fn main() -> ExitCode {
    env_logger::init();
    let cli = Cli::parse();
    let mut walk = WalkBuilder::new(&cli.root_dir);
    let walk = walk
        .git_ignore(cli.git_ignore)
        .threads(cli.threads);

    if !cli.sequential {
        walk.build_parallel().visit(&mut MyWalkerBuilder::new(&cli));
    } else {
        for file in walk.build() {
            match file {
                Ok(entry) => { process(entry, &cli).unwrap(); },
                Err(ex) => {
                    log::error!("Unexpected error: {}", ex);
                    return ExitCode::FAILURE
                },
            }

        }
    }

    ExitCode::SUCCESS
}
