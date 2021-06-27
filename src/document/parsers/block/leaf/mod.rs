mod paragraph;
mod thematic_break;

use super::{
    Document, IndividualMatcher, Matcher, Parse, ParseResult, SelectionMatcher, TryParse,
    TryParseResult,
};
use paragraph::Paragraph;
use thematic_break::ThematicBreak;

#[derive(Eq, PartialEq)]
pub enum LeafCategory {
    ThematicBreak,
    Paragraph,
}

pub struct Leaf {
    pub category: LeafCategory,
    pub text: String,
}

impl Leaf {
    fn new(category: LeafCategory, text: String) -> Leaf {
        Leaf { category, text }
    }
}

impl Parse<Leaf> for Leaf {
    fn parse(input: String, document: &Document) -> ParseResult<Leaf> {
        // TODO

        match ThematicBreak::try_parse(input, document) {
            TryParseResult::Accepted(ParseResult(
                matched_thematic_break,
                leftover_from_thematic_break,
            )) => ParseResult(
                Leaf::new(LeafCategory::ThematicBreak, matched_thematic_break),
                leftover_from_thematic_break,
            ),
            TryParseResult::Rejected(rejected_from_thematic_break_matcher) => {
                let ParseResult(matched_paragraph, leftover_from_paragraph) =
                    Paragraph::parse(rejected_from_thematic_break_matcher, document);
                ParseResult(
                    Leaf::new(LeafCategory::Paragraph, matched_paragraph),
                    leftover_from_paragraph,
                )
            }
        }
    }
}
