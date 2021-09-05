mod matchers;
mod parsers;
mod preliminaries;
mod renderers;

use matchers::{
    individual::IndividualMatcher,
    selection::SelectionMatcher,
    utils::matcher::{LeftoverString, MatchedString, Matcher, RejectedString},
};
use parsers::{
    block::{
        container::Container,
        leaf::{atx_heading::AtxHeadingLevel, LeafCategory},
        Block,
    },
    inline::{Inline, InlineCategory},
    ParseMultiple,
};
use preliminaries::Preliminaries;
use renderers::{
    normal_element::NormalElement, text_element::TextElement, void_element::VoidElement, Render,
};
use std::collections::HashMap;

pub struct Document {
    link_reference_map: HashMap<String, String>,
    preliminaries: Preliminaries,
}

impl Document {
    pub fn new() -> Document {
        Document {
            link_reference_map: HashMap::new(),
            preliminaries: Preliminaries::initialize(),
        }
    }

    pub fn render(&self, input: String) -> String {
        let block_structure = Block::parse_multiple(input, &self);

        self.render_blocks(block_structure)
    }

    fn render_blocks(&self, blocks: Vec<Block>) -> String {
        let mut rendered_blocks = String::new();

        for block in blocks {
            rendered_blocks = [rendered_blocks, self.render_block(block)].concat();
        }

        rendered_blocks
    }

    fn render_block(&self, block: Block) -> String {
        let renderer: Box<dyn Render> = match block {
            Block::Container(container) => match container {
                Container::BlockQuote(child_blocks) => Box::new(NormalElement::new(
                    String::from("blockquote"),
                    self.render_blocks(child_blocks),
                )),
            },

            Block::Leaf(leaf) => match leaf.category {
                LeafCategory::AtxHeading(atx_heading_level) => {
                    let inner_text = self.render_inlines(leaf.text);

                    match atx_heading_level {
                        AtxHeadingLevel::One => {
                            Box::new(NormalElement::new(String::from("h1"), inner_text))
                        }
                        AtxHeadingLevel::Two => {
                            Box::new(NormalElement::new(String::from("h2"), inner_text))
                        }
                        AtxHeadingLevel::Three => {
                            Box::new(NormalElement::new(String::from("h3"), inner_text))
                        }
                        AtxHeadingLevel::Four => {
                            Box::new(NormalElement::new(String::from("h4"), inner_text))
                        }
                        AtxHeadingLevel::Five => {
                            Box::new(NormalElement::new(String::from("h5"), inner_text))
                        }
                        AtxHeadingLevel::Six => {
                            Box::new(NormalElement::new(String::from("h6"), inner_text))
                        }
                    }
                }
                LeafCategory::ThematicBreak => Box::new(VoidElement::new(String::from("hr"))),
                LeafCategory::Paragraph => Box::new(NormalElement::new(
                    String::from("p"),
                    self.render_inlines(leaf.text),
                )),
            },
        };

        renderer.render()
    }

    fn render_inlines(&self, text: String) -> String {
        let mut rendered_inlines = String::new();

        for inline in Inline::parse_multiple(text, &self) {
            rendered_inlines = [rendered_inlines, self.render_inline(inline)].concat();
        }

        rendered_inlines
    }

    fn render_inline(&self, inline: Inline) -> String {
        let renderer: Box<dyn Render> = match inline.category {
            InlineCategory::CodeSpan => {
                Box::new(NormalElement::new(String::from("code"), inline.text))
            }
            InlineCategory::TextualContent => Box::new(TextElement(inline.text)),
        };

        renderer.render()
    }
}

// TODO - tests
