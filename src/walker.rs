use chrono::DateTime;
use std::fs;
use std::io;
use std::os::linux::fs::MetadataExt;
use std::path::Path;

use ignore::{DirEntry, ParallelVisitor, ParallelVisitorBuilder, WalkState};
use md5::Digest;
use md5::Md5;
use url::Url;

use crate::Cli;

const DATETIME_FORMATTER: &'static str = "%Y-%m-%dT%H:%M:%S";

pub struct MyWalkerBuilder<'a> {
    cli: &'a Cli,
}

impl<'a> MyWalkerBuilder<'a> {
    pub fn new(cli: &'a Cli) -> Self {
        MyWalkerBuilder { cli }
    }
}

impl<'a> ParallelVisitorBuilder<'a> for MyWalkerBuilder<'a> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 'a> {
        Box::new(MyWalker::new(self.cli))
    }
}

pub struct MyWalker<'a> {
    cli: &'a Cli,
}

impl<'a> MyWalker<'a> {
    pub fn new(cli: &'a Cli) -> Self {
        MyWalker { cli }
    }
}

impl<'a> ParallelVisitor for MyWalker<'a> {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        match self.do_visit(entry) {
            Ok(s) => s,
            Err(ex) => {
                log::error!("Unexpected error: {}", ex);
                WalkState::Quit
            }
        }
    }
}

impl<'a> MyWalker<'a> {
    fn do_visit(
        &mut self,
        entry: Result<ignore::DirEntry, ignore::Error>,
    ) -> Result<WalkState, anyhow::Error> {
        match entry {
            Ok(file) => {
                process(file, self.cli)?;
                Ok(WalkState::Continue)
            }
            Err(ex) => Err(ex.into()),
        }
    }
}

pub fn process(file: DirEntry, cli: &Cli) -> Result<(), anyhow::Error> {
    let st = file.metadata()?;
    if st.is_file() {
        let p = file.path().display().to_string();
        let p = p.strip_prefix(&cli.root_dir.display().to_string()).unwrap_or(&p);
        let url = Url::parse(cli.base_url.as_str())?
            .join(&p)
            .unwrap()
            .to_string();
        let hash = compute_string_hash(&url);
        let mtime: DateTime<chrono::Local> = st.modified()?.into();

        let path = file.path();
        let file_size = st.st_size();
        let mut ret: heapless::LinearMap<&str, String, 8> = [
            ("url", url),
            ("id", hash),
            ("path", fs::canonicalize(&path)?.display().to_string()),
            ("mtime", mtime.format(DATETIME_FORMATTER).to_string()),
            ("size", format!("{}", file_size)),
            ("file_size", human_bytes::human_bytes(file_size as f64))
        ].into_iter().collect();

        if cli.file_hash {
            ret.insert("file_hash", compute_file_hash(file.path())?).unwrap();
        }

        serde_json::to_writer(std::io::stdout().lock(), &ret).unwrap();
    }
    Ok(())
}

fn compute_file_hash(path: &Path) -> Result<String, anyhow::Error> {
    let mut file = fs::File::open(&path)?;
    let mut hash = Md5::default();
    io::copy(&mut file, &mut hash)?;
    Ok(format!("{:x}", hash.finalize()))
}

fn compute_string_hash(s: &String) -> String {
    let mut hash = Md5::default();
    hash.update(s);
    format!("{:x}", hash.finalize())
}
