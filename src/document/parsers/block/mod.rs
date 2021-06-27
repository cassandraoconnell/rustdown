pub mod container;
pub mod leaf;

use super::{
    Document, IndividualMatcher, Matcher, Parse, ParseMultiple, ParseResult, SelectionMatcher,
    TryParse, TryParseResult,
};
use container::Container;
use leaf::{Leaf, LeafCategory};

pub enum Block {
    Container(Container),
    Leaf(Leaf),
}

impl Parse<Block> for Block {
    fn parse(input: String, document: &Document) -> ParseResult<Block> {
        match Container::try_parse(input, document) {
            TryParseResult::Accepted(parse_result) => {
                let ParseResult(container, unconsumed) = parse_result;
                ParseResult(Block::Container(container), unconsumed)
            }
            TryParseResult::Rejected(rejected) => {
                let ParseResult(leaf, unconsumed) = Leaf::parse(rejected, document);
                ParseResult(Block::Leaf(leaf), unconsumed)
            }
        }
    }
}

impl ParseMultiple<Block> for Block {
    fn parse_multiple(input: String, document: &Document) -> Vec<Block> {
        let mut blocks = Vec::new();
        let mut optional_unconsumed = Some(input);

        loop {
            if let Some(unconsumed) = optional_unconsumed {
                let ParseResult(block, optional_leftover) = Block::parse(unconsumed, document);

                blocks.push(block);
                optional_unconsumed = optional_leftover;
            } else {
                break;
            }
        }

        blocks
    }
}
