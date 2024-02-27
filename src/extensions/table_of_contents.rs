use markdown_it::parser::core::{CoreRule, Root};
use markdown_it::parser::extset::RootExt;
use markdown_it::plugins::cmark::block::heading::ATXHeading;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct ToCList {
    pub elements: Vec<ToCElement>,
}
impl RootExt for ToCList {}

#[derive(Debug)]
pub struct ToCElement {
    pub id: String,
    pub level: u32,
    pub text: String,
}

// This defines how your custom node should be rendered.
impl NodeValue for ToCList {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        if self.elements.is_empty() {
            return;
        }

        let attrs = node.attrs.clone();

        fmt.open("ul", &attrs);
        for element in self.elements.iter() {
            let list_attrs = vec![("class", format!("level-{}", element.level))];

            fmt.open("li", &list_attrs);
            fmt.open("a", &[("href", format!("#{}", element.id))]);
            fmt.text(&element.text);
            fmt.close("a");
            fmt.close("li");
        }
        fmt.close("ul");
    }
}

struct ToCCounterRule;

impl CoreRule for ToCCounterRule {
    // a custom function that will be invoked once per document.
    fn run(root: &mut Node, _: &MarkdownIt) {
        let mut counter = 1;
        let mut toc_list = Vec::new();

        // walk through AST recursively
        root.walk_mut(|node, _| {
            if let Some(heading) = node.cast::<ATXHeading>() {
                let level = heading.level;

                // append unique id to the heading node
                let id = format!("heading-{}", counter);
                node.attrs.push(("id", id.clone().into()));

                // collect text recursively
                let text = node.collect_text();
                let element = ToCElement {
                    id,
                    level: level.into(),
                    text,
                };
                toc_list.push(element);
                counter += 1;
            }
        });

        // append ToC list Node to the root ext set
        let root_data = root.cast_mut::<Root>().unwrap();
        root_data.ext.insert(ToCList { elements: toc_list });
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<ToCCounterRule>();
}
