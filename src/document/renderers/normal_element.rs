use super::Render;

// [SPEC]: https://html.spec.whatwg.org/multipage/syntax.html#elements-2
pub struct NormalElement {
    attributes: Vec<(String, String)>,
    inner_text: String,
    tag: String,
}

impl NormalElement {
    pub fn new(tag: String, inner_text: String) -> NormalElement {
        NormalElement {
            attributes: Vec::new(),
            inner_text,
            tag,
        }
    }

    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.push((key, value));
    }
}

impl Render for NormalElement {
    fn render(&self) -> String {
        format!(
            "<{opening_tag}>{inner_text}</{closing_tag}>",
            opening_tag = self.tag,
            inner_text = self.inner_text,
            closing_tag = self.tag
        )
    }
}
