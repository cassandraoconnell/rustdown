mod block_quote;

use crate::document::parsers::ParseMultiple;

use super::{
    Block, Document, Leaf, LeafCategory, Matcher, Parse, ParseResult, SelectionMatcher, TryParse,
    TryParseResult,
};
use block_quote::BlockQuote;

pub enum Container {
    BlockQuote(Vec<Block>),
    // List(Vec<Block>),
    // ListItem(Vec<Block>),
}

impl TryParse<Container> for Container {
    fn try_parse(input: String, document: &Document) -> TryParseResult<Container> {
        match BlockQuote::try_parse(input, document) {
            TryParseResult::Accepted(ParseResult(consumed, unconsumed)) => {
                TryParseResult::Accepted(ParseResult(
                    Container::BlockQuote(Block::parse_multiple(consumed, document)),
                    unconsumed,
                ))
            }
            TryParseResult::Rejected(rejected) => {
                // TODO

                TryParseResult::Rejected(rejected)
            }
        }
    }
}
