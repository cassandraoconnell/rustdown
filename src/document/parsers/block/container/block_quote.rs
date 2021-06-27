use super::{
    Document, Leaf, LeafCategory, Matcher, Parse, ParseResult, SelectionMatcher, TryParse,
    TryParseResult,
};

pub struct BlockQuote;

impl TryParse<String> for BlockQuote {
    fn try_parse(input: String, document: &Document) -> TryParseResult<String> {
        let delimiter_matcher = SelectionMatcher::from(vec![String::from("> "), String::from('>')]);

        let mut is_block_quote = false;

        let mut consumed = String::new();
        let mut unconsumed = input;

        let mut leading_space_count = 0;

        while !unconsumed.is_empty() {
            if !is_block_quote {
                match delimiter_matcher.try_match(unconsumed) {
                    Ok((_, leftover_from_matched_delimiter)) => {
                        unconsumed = leftover_from_matched_delimiter;
                        is_block_quote = true;

                        if leading_space_count > 0 {
                            consumed = String::new();
                        }

                        continue;
                    }
                    Err(rejected_from_delimiter_matcher) => {
                        if leading_space_count < 3 {
                            match document
                                .preliminaries
                                .space
                                .try_match(rejected_from_delimiter_matcher)
                            {
                                Ok((matched_space, leftover_from_matched_space)) => {
                                    leading_space_count = leading_space_count + 1;
                                    consumed = [consumed, matched_space].concat();
                                    unconsumed = leftover_from_matched_space;
                                    continue;
                                }
                                Err(rejected_from_space_matcher) => {
                                    unconsumed = rejected_from_space_matcher;
                                    break;
                                }
                            }
                        } else {
                            unconsumed = rejected_from_delimiter_matcher;
                            break;
                        }
                    }
                }
            } else {
                match document.preliminaries.line.try_match(unconsumed) {
                    Ok((matched_line, leftover_from_line_matcher)) => {
                        match document.preliminaries.blank_line.try_match(matched_line) {
                            Ok((_, leftover_from_blank_line_matcher)) => {
                                unconsumed = leftover_from_blank_line_matcher;
                                break;
                            }
                            Err(rejected_from_blank_line_matcher) => {
                                match delimiter_matcher.try_match(rejected_from_blank_line_matcher)
                                {
                                    Ok((_, leftover_from_matched_delimiter)) => {
                                        consumed =
                                            [consumed, leftover_from_matched_delimiter].concat();
                                        unconsumed = leftover_from_line_matcher;
                                        continue;
                                    }
                                    Err(rejected_from_delimiter_matcher) => {
                                        let ParseResult(leaf, optional_leftover_from_leaf_parser) =
                                            Leaf::parse(rejected_from_delimiter_matcher, document);

                                        if leaf.category == LeafCategory::Paragraph {
                                            consumed = [consumed, leaf.text].concat();
                                            unconsumed = match optional_leftover_from_leaf_parser {
                                                Some(leftover_from_leaf_parser) => [
                                                    leftover_from_leaf_parser,
                                                    leftover_from_line_matcher,
                                                ]
                                                .concat(),
                                                None => leftover_from_line_matcher,
                                            };

                                            continue;
                                        } else {
                                            unconsumed = match optional_leftover_from_leaf_parser {
                                                Some(leftover_from_leaf_parser) => [
                                                    leaf.text,
                                                    leftover_from_leaf_parser,
                                                    leftover_from_line_matcher,
                                                ]
                                                .concat(),
                                                None => {
                                                    [leaf.text, leftover_from_line_matcher].concat()
                                                }
                                            };
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(rejected_from_line_matcher) => {
                        unconsumed = rejected_from_line_matcher;
                        break;
                    }
                }
            }
        }

        if is_block_quote {
            return TryParseResult::Accepted(ParseResult::new(consumed, unconsumed));
        } else {
            return TryParseResult::Rejected([consumed, unconsumed].concat());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockQuote, Document, ParseResult, TryParse, TryParseResult};

    #[test]
    fn it_accepts_simple_example() {
        let document = Document::new();

        assert_eq!(
            BlockQuote::try_parse(String::from("> # Foo\n> bar\n> baz"), &document),
            TryParseResult::Accepted(ParseResult(String::from("# Foo\nbar\nbaz"), None))
        );
    }

    #[test]
    fn it_accepts_omitted_space_after_delimiter() {
        let document = Document::new();

        assert_eq!(
            BlockQuote::try_parse(String::from("> # Foo\n>bar\n> baz"), &document),
            TryParseResult::Accepted(ParseResult(String::from("# Foo\nbar\nbaz"), None))
        );
    }

    #[test]
    fn it_accepts_one_leading_space() {
        let document = Document::new();

        assert_eq!(
            BlockQuote::try_parse(String::from(" > # Foo"), &document),
            TryParseResult::Accepted(ParseResult(String::from("# Foo"), None))
        );
    }

    #[test]
    fn it_accepts_two_leading_spaces() {
        let document = Document::new();

        assert_eq!(
            BlockQuote::try_parse(String::from("  > # Foo"), &document),
            TryParseResult::Accepted(ParseResult(String::from("# Foo"), None))
        );
    }

    #[test]
    fn it_accepts_three_leading_spaces() {
        let document = Document::new();

        assert_eq!(
            BlockQuote::try_parse(String::from("   > # Foo"), &document),
            TryParseResult::Accepted(ParseResult(String::from("# Foo"), None))
        );
    }

    #[test]
    fn it_rejects_four_or_more_leading_spaces() {
        let document = Document::new();

        assert_eq!(
            BlockQuote::try_parse(String::from("    > # Foo"), &document),
            TryParseResult::Rejected(String::from("    > # Foo"))
        );

        assert_eq!(
            BlockQuote::try_parse(String::from("     > # Foo"), &document),
            TryParseResult::Rejected(String::from("     > # Foo"))
        );
    }

    #[test]
    fn it_accepts_lazy_continuation_lines() {
        let document = Document::new();

        assert_eq!(
            BlockQuote::try_parse(String::from("> # Foo\nbar\nbaz"), &document),
            TryParseResult::Accepted(ParseResult(String::from("# Foo\nbar\nbaz"), None))
        );
    }

    #[test]
    fn it_accepts_some_lazy_continuation_lines_and_some_delimiters() {
        let document = Document::new();

        assert_eq!(
            BlockQuote::try_parse(String::from("> # Foo\nbar\n>baz"), &document),
            TryParseResult::Accepted(ParseResult(String::from("# Foo\nbar\nbaz"), None))
        );
    }

    #[test]
    fn it_rejects_lazy_continuation_lines_for_non_paragraph_continuation_text() {
        let document = Document::new();

        assert_eq!(
            BlockQuote::try_parse(String::from("> # Foo\n---"), &document),
            TryParseResult::Accepted(ParseResult(
                String::from("# Foo\n"),
                Some(String::from("---"))
            ))
        );
    }
}
