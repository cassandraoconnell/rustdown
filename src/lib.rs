pub mod document;

use document::Document;

pub fn parse(input: String) -> String {
    let document = Document::new();
    document.render(input)
}
