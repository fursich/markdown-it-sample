use crate::extensions;
use crate::extensions::table_of_contents::ToCList;
use markdown_it::parser::core::Root;
use markdown_it::plugins::{cmark, extra};
use markdown_it::{MarkdownIt, Node};
use std::sync::OnceLock;

pub(super) struct MarkdownDriver {
    md: MarkdownIt,
    source: OnceLock<String>,
    contents: OnceLock<Node>,
    toc: OnceLock<Node>,
}

impl MarkdownDriver {
    pub(super) fn new() -> Self {
        // create markdown parser
        let mut md = MarkdownIt::new();
        Self::prepare(&mut md);

        Self {
            md,
            source: OnceLock::new(),
            contents: OnceLock::new(),
            toc: OnceLock::new(),
        }
    }

    pub(super) fn parse(&self, contents: String) {
        if self.source.set(contents.clone()).is_err() {
            return;
        }

        let mut root = self.md.parse(contents.as_str());
        // take out ToCList from the root ext set
        let root_data = root.cast_mut::<Root>().unwrap();
        let toc = root_data.ext.remove::<ToCList>().unwrap();

        self.contents.set(root).unwrap();
        self.toc.set(Node::new(toc)).unwrap();
    }

    pub(super) fn render(&self) -> String {
        let contents = self.contents.get();
        match contents {
            None => String::new(),
            Some(contents) => contents.render(),
        }
    }

    pub(super) fn render_toc(&self) -> String {
        let toc = self.toc.get();
        match toc {
            None => String::new(),
            Some(toc) => toc.render(),
        }
    }

    fn prepare(md: &mut MarkdownIt) {
        cmark::add(md);
        extra::add(md);

        // add custom three rules described above
        extensions::highlight::add(md);
        extensions::table_of_contents::add(md);
    }
}
