use super::{
    Document, IndividualMatcher, Matcher, ParseResult, SelectionMatcher, TryParse, TryParseResult,
};

#[derive(Debug, Eq, PartialEq)]
pub struct ThematicBreak;

impl TryParse<String> for ThematicBreak {
    fn try_parse(input: String, document: &Document) -> TryParseResult<String> {
        let delimiter_selection_matcher = SelectionMatcher::from(vec![
            String::from('-'),
            String::from('_'),
            String::from('*'),
        ]);

        let mut optional_delimiter_individual_matcher: Option<Box<dyn Matcher>> = None;
        let mut delimiter_count = 0;
        let mut leading_space_count = 0;

        let mut consumed = String::new();
        let mut unconsumed = input;

        let mut is_thematic_break = false;

        while !unconsumed.is_empty() {
            match document.preliminaries.line_ending.try_match(unconsumed) {
                Ok((matched_line_ending, line_ending_leftover)) => {
                    consumed = [consumed, matched_line_ending].concat();
                    unconsumed = line_ending_leftover;
                    break;
                }
                Err(rejected_line_ending) => match &optional_delimiter_individual_matcher {
                    None => match delimiter_selection_matcher.try_match(rejected_line_ending) {
                        Ok((matched_delimiter, delimiter_leftover)) => {
                            optional_delimiter_individual_matcher = Some(Box::new(
                                IndividualMatcher::from(String::from(&matched_delimiter)),
                            ));

                            delimiter_count = delimiter_count + 1;
                            consumed = [consumed, matched_delimiter].concat();
                            unconsumed = delimiter_leftover;
                            continue;
                        }
                        Err(rejected_delimiter) => {
                            if leading_space_count < 3 {
                                match document.preliminaries.space.try_match(rejected_delimiter) {
                                    Ok((matched_space, space_leftover)) => {
                                        consumed = [consumed, matched_space].concat();
                                        leading_space_count = leading_space_count + 1;
                                        unconsumed = space_leftover;
                                        continue;
                                    }
                                    Err(rejected_space) => {
                                        unconsumed = rejected_space;
                                        break;
                                    }
                                }
                            }

                            unconsumed = rejected_delimiter;
                            break;
                        }
                    },
                    Some(delimiter_individual_matcher) => {
                        match delimiter_individual_matcher.try_match(rejected_line_ending) {
                            Ok((delimiter_match_result, leftover_from_delimiter)) => {
                                delimiter_count = delimiter_count + 1;
                                consumed = [consumed, delimiter_match_result].concat();
                                unconsumed = leftover_from_delimiter;

                                if delimiter_count == 3 {
                                    is_thematic_break = true;
                                }

                                continue;
                            }
                            Err(rejected_delimiter) => {
                                match document.preliminaries.space.try_match(rejected_delimiter) {
                                    Ok((matched_space, space_leftover)) => {
                                        consumed = [consumed, matched_space].concat();
                                        unconsumed = space_leftover;
                                        continue;
                                    }
                                    Err(rejected_space) => {
                                        is_thematic_break = false;
                                        unconsumed = rejected_space;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }

        if is_thematic_break {
            return TryParseResult::Accepted(ParseResult::new(consumed, unconsumed));
        } else {
            return TryParseResult::Rejected([consumed, unconsumed].concat());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Document, ParseResult, ThematicBreak, TryParse, TryParseResult};

    #[test]
    fn it_accepts_three_subsequent_asterisks() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("***"), &document),
            TryParseResult::Accepted(ParseResult(String::from("***"), None))
        );
    }

    #[test]
    fn it_accepts_three_subsequent_dashes() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("---"), &document),
            TryParseResult::Accepted(ParseResult(String::from("---"), None))
        );
    }

    #[test]
    fn it_accepts_three_subsequent_underscores() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("___"), &document),
            TryParseResult::Accepted(ParseResult(String::from("___"), None))
        );
    }

    #[test]
    fn it_accepts_basic_example_and_returns_remaining_lines() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("***\nleftover"), &document),
            TryParseResult::Accepted(ParseResult(
                String::from("***\n"),
                Some(String::from("leftover"))
            ))
        );
    }

    #[test]
    fn it_rejects_wrong_characters() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("+++"), &document),
            TryParseResult::Rejected(String::from("+++"))
        );

        assert_eq!(
            ThematicBreak::try_parse(String::from("==="), &document),
            TryParseResult::Rejected(String::from("==="))
        );
    }

    #[test]
    fn it_rejects_not_enough_characters() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("**"), &document),
            TryParseResult::Rejected(String::from("**"))
        );
    }

    #[test]
    fn it_accepts_single_space_indent() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from(" ***"), &document),
            TryParseResult::Accepted(ParseResult(String::from(" ***"), None))
        );
    }

    #[test]
    fn it_accepts_double_space_indent() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("  ***"), &document),
            TryParseResult::Accepted(ParseResult(String::from("  ***"), None))
        );
    }

    #[test]
    fn it_accepts_triple_space_indent() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("   ***"), &document),
            TryParseResult::Accepted(ParseResult(String::from("   ***"), None))
        );
    }

    #[test]
    fn it_rejects_quadruple_space_indent() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("    ***"), &document),
            TryParseResult::Rejected(String::from("    ***"))
        );
    }

    #[test]
    fn it_accepts_four_or_more_delimiters() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("****"), &document),
            TryParseResult::Accepted(ParseResult(String::from("****"), None))
        );
    }

    #[test]
    fn it_accepts_spaces_between_delimiters() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("* * *"), &document),
            TryParseResult::Accepted(ParseResult(String::from("* * *"), None))
        );
    }

    #[test]
    fn it_rejects_other_characters_on_line() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("***a"), &document),
            TryParseResult::Rejected(String::from("***a"))
        );
    }

    #[test]
    fn it_rejects_multiple_different_correct_delimiters() {
        let document = Document::new();

        assert_eq!(
            ThematicBreak::try_parse(String::from("*-*"), &document),
            TryParseResult::Rejected(String::from("*-*"))
        );
    }
}
