mod driver;
mod extensions;

use driver::MarkdownDriver;
use std::fs;

#[allow(dead_code)]
fn convert(contents: String) -> String {
    let handler = MarkdownDriver::new();
    handler.parse(contents);
    handler.render()
}

#[allow(dead_code)]
fn convert_with_toc(contents: String) -> (String, String) {
    let handler = MarkdownDriver::new();
    handler.parse(contents);
    (handler.render(), handler.render_toc())
}

fn main() {
    let contents = fs::read_to_string("src/fixtures/sample.md").expect("Unable to read file");

    let (body, toc) = convert_with_toc(contents.to_string());

    println!("<h2> 目次 </h2>\n<hr>");
    println!("{}", toc);
    println!("<hr>");

    println!("{}", body);
}
