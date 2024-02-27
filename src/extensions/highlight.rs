use markdown_it::generics::inline::emph_pair;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct Highlight {
    pub marker: char,
}

impl NodeValue for Highlight {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.open("mark", &node.attrs);
        fmt.contents(&node.children);
        fmt.close("mark");
    }
}

pub fn add(md: &mut MarkdownIt) {
    emph_pair::add_with::<'=', 2, false>(md, || Node::new(Highlight { marker: '=' }));
}
