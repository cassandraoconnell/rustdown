mod blank_line;
mod character;
mod line;
mod line_ending;
mod space;
mod whitespace;

use super::{
    IndividualMatcher, LeftoverString, MatchedString, Matcher, RejectedString, SelectionMatcher,
};
use blank_line::BlankLine;
use character::Character;
use line::Line;
use line_ending::LineEnding;
use space::Space;
use whitespace::Whitespace;

const CARRIAGE_RETURN: char = '\u{000D}';
const FORM_FEED: char = '\u{000C}';
const LINE_TABULATION: char = '\u{000B}';
const NEWLINE: char = '\u{000A}';
const SPACE: char = '\u{0020}';
const TAB: char = '\u{0009}';

pub struct Preliminaries {
    pub blank_line: BlankLine,
    pub character: Character,
    pub line: Line,
    pub line_ending: LineEnding,
    pub space: Space,
    pub whitespace: Whitespace,
}

impl Preliminaries {
    pub fn initialize() -> Preliminaries {
        Preliminaries {
            blank_line: BlankLine,
            character: Character,
            line: Line,
            line_ending: LineEnding,
            space: Space,
            whitespace: Whitespace,
        }
    }
}
