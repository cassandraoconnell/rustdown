use super::Render;

// [SPEC]: https://html.spec.whatwg.org/multipage/syntax.html#elements-2
pub struct TextElement(pub String);

impl Render for TextElement {
    fn render(&self) -> String {
        let TextElement(text) = self;

        String::from(text)
    }
}
