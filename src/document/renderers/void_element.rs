use super::Render;

// [SPEC]: https://html.spec.whatwg.org/multipage/syntax.html#elements-2
pub struct VoidElement {
    attributes: Vec<(String, String)>,
    tag: String,
}

impl VoidElement {
    pub fn new(tag: String) -> VoidElement {
        VoidElement {
            attributes: Vec::new(),
            tag,
        }
    }

    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.push((key, value));
    }
}

impl Render for VoidElement {
    fn render(&self) -> String {
        format!("<{tag} />", tag = self.tag)
    }
}
