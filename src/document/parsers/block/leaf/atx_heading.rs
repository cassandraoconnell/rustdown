use super::{
    Document, IndividualMatcher, Matcher, ParseResult, SelectionMatcher, TryParse, TryParseResult,
};

#[derive(Debug, Eq, PartialEq)]

pub enum AtxHeadingLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
pub struct AtxHeading {
    level: AtxHeadingLevel,
}

impl TryParse<(AtxHeadingLevel, String)> for AtxHeading {
    fn try_parse(input: String, document: &Document) -> TryParseResult<(AtxHeadingLevel, String)> {
        // TODO

        TryParseResult::Rejected(input)
    }
}
