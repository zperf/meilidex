use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};
use serde_json::json;
use url::Url;

pub struct MyWalkerBuilder {
    base_url: String,
    prefix: String,
}

impl MyWalkerBuilder {
    pub fn new(base_url: &String, prefix: &String) -> Self {
        MyWalkerBuilder {
            base_url: base_url.to_string(),
            prefix: prefix.to_string(),
        }
    }
}

impl<'a> ParallelVisitorBuilder<'a> for MyWalkerBuilder {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 'a> {
        Box::new(MyWalker::new(&self))
    }
}

pub struct MyWalker {
    base_url: Url,
    prefix: String,
}

impl MyWalker {
    pub fn new(builder: &MyWalkerBuilder) -> Self {
        MyWalker {
            base_url: Url::parse(builder.base_url.as_str()).unwrap(),
            prefix: builder.prefix.to_string(),
        }
    }
}

impl ParallelVisitor for MyWalker {
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

impl MyWalker {
    fn do_visit(
        &mut self,
        entry: Result<ignore::DirEntry, ignore::Error>,
    ) -> Result<WalkState, anyhow::Error> {
        match entry {
            Ok(file) => {
                if file.metadata()?.is_file() {
                    let p = file.path().display().to_string();
                    let p = p.strip_prefix(&self.prefix).unwrap_or(&p);
                    let url = self.base_url.clone().join(&p).unwrap().to_string();
                    let hash = md5::compute(&url);
                    println!(
                        "{}",
                        serde_json::to_string(&json!({
                            "url": url,
                            "hash": format!("{:x}", hash),
                        }))
                        .unwrap()
                    );
                }
                Ok(WalkState::Continue)
            }
            Err(ex) => Err(ex.into()),
        }
    }
}
