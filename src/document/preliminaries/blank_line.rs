use super::{
    LeftoverString, LineEnding, MatchedString, Matcher, RejectedString, SelectionMatcher, SPACE,
    TAB,
};

pub struct BlankLine;

impl Matcher for BlankLine {
    fn try_match(&self, input: String) -> Result<(MatchedString, LeftoverString), RejectedString> {
        let mut consumed = String::new();
        let mut unconsumed = input;

        let line_ending = LineEnding;
        let space_or_tab = SelectionMatcher::from(vec![String::from(SPACE), String::from(TAB)]);

        let mut is_blank_line = true;

        while !unconsumed.is_empty() {
            match line_ending.try_match(unconsumed) {
                Ok((matched_line_ending, leftover_from_line_ending)) => {
                    consumed = [consumed, matched_line_ending].concat();
                    unconsumed = leftover_from_line_ending;
                    break;
                }
                Err(rejected_from_line_ending) => {
                    match space_or_tab.try_match(rejected_from_line_ending) {
                        Ok((matched_space_or_tab, leftover_from_space_or_tab)) => {
                            consumed = [consumed, matched_space_or_tab].concat();
                            unconsumed = leftover_from_space_or_tab;
                            continue;
                        }
                        Err(rejected_from_space_or_tab) => {
                            is_blank_line = false;
                            unconsumed = rejected_from_space_or_tab;
                            break;
                        }
                    }
                }
            }
        }

        if is_blank_line {
            return Ok((consumed, unconsumed));
        }

        Err([consumed, unconsumed].concat())
    }
}

#[cfg(test)]
mod tests {
    use super::{BlankLine, Matcher};

    #[test]
    fn it_accepts_empty_line() {
        let blank_line = BlankLine;

        assert_eq!(
            blank_line.try_match(String::from("\nline\n")),
            Ok((String::from("\n"), String::from("line\n")))
        );
    }

    #[test]
    fn it_accepts_line_with_spaces_and_tabs() {
        let blank_line = BlankLine;

        assert_eq!(
            blank_line.try_match(String::from("      \nline\n")),
            Ok((String::from("      \n"), String::from("line\n")))
        );
    }

    #[test]
    fn it_rejects_lines_with_wrong_characters() {
        let blank_line = BlankLine;

        assert_eq!(
            blank_line.try_match(String::from("   a\n")),
            Err(String::from("   a\n"))
        );
    }
}
