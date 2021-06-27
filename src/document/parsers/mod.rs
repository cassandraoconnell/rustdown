pub mod block;
pub mod inline;
pub mod utils;

use super::{Document, IndividualMatcher, Matcher, SelectionMatcher};

#[derive(Debug, Eq, PartialEq)]
pub struct ParseResult<T>(pub T, pub Option<String>);

impl<T> ParseResult<T> {
    fn new(result: T, unconsumed: String) -> ParseResult<T> {
        let optional_leftover = match unconsumed.is_empty() {
            false => Some(unconsumed),
            true => None,
        };

        ParseResult(result, optional_leftover)
    }
}

pub trait Parse<T> {
    fn parse(input: String, document: &Document) -> ParseResult<T>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum TryParseResult<T> {
    Accepted(ParseResult<T>),
    Rejected(String),
}

pub trait TryParse<T> {
    fn try_parse(input: String, document: &Document) -> TryParseResult<T>;
}

pub trait ParseMultiple<T> {
    fn parse_multiple(input: String, document: &Document) -> Vec<T>;
}
