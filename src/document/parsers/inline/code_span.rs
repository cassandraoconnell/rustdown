use super::{Document, IndividualMatcher, Matcher, ParseResult, TryParse, TryParseResult};

pub struct CodeSpan;

const CODE_SPAN_DELIMITER: char = '`';

impl TryParse<String> for CodeSpan {
    fn try_parse(input: String, document: &Document) -> TryParseResult<String> {
        let delimiter_matcher = IndividualMatcher::from(CODE_SPAN_DELIMITER);

        let mut is_code_span = false;

        let mut consumed = String::new();
        let mut unconsumed = input;

        let mut is_leading_delimiter_run = true;
        let mut leading_delimiter_count = 0;
        let mut trailing_delimiter_count = 0;

        let mut has_leading_space = false;
        let mut has_trailing_space = false;

        while !unconsumed.is_empty() {
            match delimiter_matcher.try_match(unconsumed) {
                Ok((matched_delimiter, leftover_from_matched_delimiter)) => {
                    is_code_span = true;
                    consumed = [consumed, matched_delimiter].concat();

                    if is_leading_delimiter_run {
                        leading_delimiter_count = leading_delimiter_count + 1;
                    } else {
                        trailing_delimiter_count = trailing_delimiter_count + 1;
                    }

                    if leading_delimiter_count == trailing_delimiter_count {
                        match delimiter_matcher.try_match(leftover_from_matched_delimiter) {
                            Ok((matched_next_delimiter, leftover_from_matched_next_delimiter)) => {
                                consumed = [consumed, matched_next_delimiter].concat();
                                unconsumed = leftover_from_matched_next_delimiter;
                                continue;
                            }
                            Err(rejected_from_delimiter_matcher) => {
                                unconsumed = rejected_from_delimiter_matcher;
                                break;
                            }
                        }
                    } else {
                        unconsumed = leftover_from_matched_delimiter;
                        continue;
                    }
                }
                Err(rejected_from_delimiter_matcher) => {
                    if is_code_span {
                        trailing_delimiter_count = 0;

                        match document
                            .preliminaries
                            .space
                            .try_match(rejected_from_delimiter_matcher)
                        {
                            Ok((matched_space, leftover_from_matched_space)) => {
                                if is_leading_delimiter_run {
                                    is_leading_delimiter_run = false;
                                    has_leading_space = true;
                                } else {
                                    has_trailing_space = true;
                                }

                                consumed = [consumed, matched_space].concat();
                                unconsumed = leftover_from_matched_space;
                                continue;
                            }
                            Err(rejected_from_space_matcher) => {
                                is_leading_delimiter_run = false;
                                has_trailing_space = false;

                                match document
                                    .preliminaries
                                    .character
                                    .try_match(rejected_from_space_matcher)
                                {
                                    Ok((matched_character, leftover_from_matched_character)) => {
                                        consumed = [consumed, matched_character].concat();
                                        unconsumed = leftover_from_matched_character;
                                        continue;
                                    }
                                    Err(rejected_from_character_matcher) => {
                                        unconsumed = rejected_from_character_matcher;
                                        break;
                                    }
                                }
                            }
                        }
                    } else {
                        unconsumed = rejected_from_delimiter_matcher;
                        break;
                    }
                }
            }
        }

        if is_code_span && leading_delimiter_count != trailing_delimiter_count {
            is_code_span = false;
        }

        if is_code_span {
            let mut delimiter_run_to_strip = String::new();
            let mut delimiter_count = leading_delimiter_count;

            while delimiter_count > 0 {
                delimiter_run_to_strip.push(CODE_SPAN_DELIMITER);
                delimiter_count = delimiter_count - 1;
            }

            let delimiter_run_matcher = IndividualMatcher::from(delimiter_run_to_strip);

            let mut normalized = String::new();

            while !consumed.is_empty() {
                match delimiter_run_matcher.try_match(consumed) {
                    Ok((matched_delimiter_run, leftover_from_matched_delimiter_run)) => {
                        match delimiter_matcher.try_match(leftover_from_matched_delimiter_run) {
                            Ok((matched_delimiter, leftover_from_matched_delimiter)) => {
                                normalized =
                                    [normalized, matched_delimiter_run, matched_delimiter].concat();
                                consumed = leftover_from_matched_delimiter;
                            }
                            Err(rejected_from_delimiter_matcher) => {
                                consumed = rejected_from_delimiter_matcher;
                            }
                        }

                        continue;
                    }
                    Err(rejected_from_delimiter_run_matcher) => {
                        match document
                            .preliminaries
                            .line_ending
                            .try_match(rejected_from_delimiter_run_matcher)
                        {
                            Ok((_, leftover_from_matched_line_ending)) => {
                                normalized =
                                    [normalized, document.preliminaries.space.get_literal()]
                                        .concat();
                                consumed = leftover_from_matched_line_ending;
                                continue;
                            }
                            Err(rejected_from_line_ending_matcher) => {
                                match document
                                    .preliminaries
                                    .character
                                    .try_match(rejected_from_line_ending_matcher)
                                {
                                    Ok((matched_character, leftover_from_matched_character)) => {
                                        normalized = [normalized, matched_character].concat();
                                        consumed = leftover_from_matched_character;
                                        continue;
                                    }
                                    Err(rejected_from_character_matcher) => {
                                        consumed = rejected_from_character_matcher;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if has_leading_space && has_trailing_space {
                let is_only_spaces = normalized.contains(|character| {
                    match document
                        .preliminaries
                        .space
                        .try_match(String::from(character))
                    {
                        Ok(_) => false,
                        Err(_) => true,
                    }
                });

                if is_only_spaces {
                    let mut normalized_chars = normalized.chars();
                    normalized_chars.next();

                    normalized = normalized_chars.collect();

                    let mut reversed_normalized_chars = normalized.chars().rev();
                    reversed_normalized_chars.next();

                    normalized = reversed_normalized_chars.collect();
                }
            }

            return TryParseResult::Accepted(ParseResult::new(
                normalized,
                [consumed, unconsumed].concat(),
            ));
        } else {
            return TryParseResult::Rejected([consumed, unconsumed].concat());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CodeSpan, Document, ParseResult, TryParse, TryParseResult};

    #[test]
    fn it_parses_basic_example() {
        let document = Document::new();

        assert_eq!(
            CodeSpan::try_parse(String::from("`foo`"), &document),
            TryParseResult::Accepted(ParseResult(String::from("foo"), None))
        );
    }

    #[test]
    fn it_parses_basic_example_with_leftover() {
        let document = Document::new();

        assert_eq!(
            CodeSpan::try_parse(String::from("`foo` bar"), &document),
            TryParseResult::Accepted(ParseResult(String::from("foo"), Some(String::from(" bar"))))
        );
    }

    #[test]
    fn it_parses_multiple_delimiters() {
        let document = Document::new();

        assert_eq!(
            CodeSpan::try_parse(String::from("``foo ` bar``"), &document),
            TryParseResult::Accepted(ParseResult(String::from("foo ` bar"), None))
        );
    }

    #[test]
    fn it_strips_single_leading_and_trailing_spaces_if_they_both_exist() {
        let document = Document::new();

        assert_eq!(
            CodeSpan::try_parse(String::from("`  ``  `"), &document),
            TryParseResult::Accepted(ParseResult(String::from(" `` "), None))
        );
    }

    #[test]
    fn it_does_not_strip_leading_space_if_trailing_space_does_not_exists() {
        let document = Document::new();

        assert_eq!(
            CodeSpan::try_parse(String::from("` a`"), &document),
            TryParseResult::Accepted(ParseResult(String::from(" a"), None))
        );
    }

    #[test]
    fn it_does_not_strip_unicode_white_space() {
        let document = Document::new();

        assert_eq!(
            CodeSpan::try_parse(String::from("`\tb\t`"), &document),
            TryParseResult::Accepted(ParseResult(String::from("\tb\t"), None))
        );
    }

    #[test]
    fn it_does_not_strip_space_if_span_only_contains_spaces() {
        let document = Document::new();

        assert_eq!(
            CodeSpan::try_parse(String::from("`  `"), &document),
            TryParseResult::Accepted(ParseResult(String::from("  "), None))
        );
    }

    #[test]
    fn it_converts_line_endings_to_spaces() {
        let document = Document::new();

        assert_eq!(
            CodeSpan::try_parse(String::from("`foo\nbar`"), &document),
            TryParseResult::Accepted(ParseResult(String::from("foo bar"), None))
        );
    }
}
