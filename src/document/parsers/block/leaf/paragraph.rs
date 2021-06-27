use super::{Document, Matcher, Parse, ParseResult};

#[derive(Debug, Eq, PartialEq)]
pub struct Paragraph;

impl Parse<String> for Paragraph {
    fn parse(input: String, document: &Document) -> ParseResult<String> {
        let mut consumed = String::new();
        let mut unconsumed = input;

        while !unconsumed.is_empty() {
            match document.preliminaries.blank_line.try_match(unconsumed) {
                Ok((_, leftover_from_matched_blank_line)) => {
                    unconsumed = leftover_from_matched_blank_line;
                    break;
                }
                Err(rejected_from_blank_line_matcher) => {
                    match document
                        .preliminaries
                        .line
                        .try_match(rejected_from_blank_line_matcher)
                    {
                        Ok((matched_line, leftover_from_line_matcher)) => {
                            consumed = [consumed, matched_line].concat();
                            unconsumed = leftover_from_line_matcher;
                            continue;
                        }
                        Err(rejected_from_line_matcher) => {
                            unconsumed = rejected_from_line_matcher;
                            break;
                        }
                    }
                }
            }
        }

        ParseResult::new(consumed, unconsumed)
    }
}

#[cfg(test)]
mod tests {
    use super::{Document, Paragraph, Parse, ParseResult};

    #[test]
    fn it_accepts_any_sequence_of_non_blank_lines() {
        let document = Document::new();

        assert_eq!(
            Paragraph::parse(
                String::from("paragraph\ncontinued paragraph\n\nsomething else"),
                &document
            ),
            ParseResult(
                String::from("paragraph\ncontinued paragraph\n"),
                Some(String::from("something else"))
            )
        );
    }
}
