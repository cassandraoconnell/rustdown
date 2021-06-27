mod code_span;

use super::{
    Document, IndividualMatcher, Matcher, Parse, ParseMultiple, ParseResult, TryParse,
    TryParseResult,
};
use code_span::CodeSpan;

#[derive(Eq, PartialEq)]
pub enum InlineCategory {
    CodeSpan,
    TextualContent,
}

pub struct Inline {
    pub category: InlineCategory,
    pub text: String,
}

impl Inline {
    fn new(category: InlineCategory, text: String) -> Inline {
        Inline { category, text }
    }
}

impl Parse<Inline> for Inline {
    fn parse(input: String, document: &Document) -> ParseResult<Inline> {
        match CodeSpan::try_parse(input, document) {
            TryParseResult::Accepted(ParseResult(matched_code_span, leftover_from_code_span)) => {
                ParseResult(
                    Inline::new(InlineCategory::CodeSpan, matched_code_span),
                    leftover_from_code_span,
                )
            }
            TryParseResult::Rejected(rejected_from_code_span_matcher) => ParseResult(
                Inline::new(
                    InlineCategory::TextualContent,
                    rejected_from_code_span_matcher,
                ),
                None,
            ),
        }
    }
}

impl ParseMultiple<Inline> for Inline {
    fn parse_multiple(input: String, document: &Document) -> Vec<Inline> {
        let mut inlines = Vec::new();
        let mut optional_unconsumed = Some(input);

        loop {
            if let Some(unconsumed) = optional_unconsumed {
                let ParseResult(inline, optional_leftover) = Inline::parse(unconsumed, document);

                inlines.push(inline);
                optional_unconsumed = optional_leftover;
            } else {
                break;
            }
        }

        inlines
    }
}
