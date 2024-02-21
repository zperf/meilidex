#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;

#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;

use chrono::DateTime;
use std::fs;
use std::io;
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
    fn visit(&mut self, entry: Result<DirEntry, ignore::Error>) -> WalkState {
        self.do_visit(entry).unwrap();
        WalkState::Continue
    }
}

impl<'a> MyWalker<'a> {
    fn do_visit(
        &mut self,
        entry: Result<DirEntry, ignore::Error>,
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
    if !st.is_file() {
        return Ok(());
    }

    let p = file.path().display().to_string();
    let p = p.strip_prefix(&cli.root_dir.display().to_string()).unwrap_or(&p);
    let url = match &cli.base_url {
        None => String::new(),
        Some(base_url) => Url::parse(base_url.as_str())?.join(&p).unwrap().to_string(),
    };
    let mtime: DateTime<chrono::Local> = st.modified()?.into();

    let path = file.path();
    let path_str = fs::canonicalize(&path)?.display().to_string();
    let hash = compute_string_hash(&path_str);
    let file_size = st.st_size();
    let mut ret: heapless::LinearMap<&str, String, 8> = [
        ("id", hash),
        ("path", path_str),
        ("mtime", mtime.format(DATETIME_FORMATTER).to_string()),
        ("file_size", human_bytes::human_bytes(file_size as f64))
    ].into_iter().collect();

    if cli.file_hash {
        ret.insert("file_hash", compute_file_hash(file.path())?).unwrap();
    }
    if !url.is_empty() {
        ret.insert("url", url).unwrap();
    }

    println!("{}", serde_json::to_string(&ret).unwrap());
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
