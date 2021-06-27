use super::{Character, LeftoverString, LineEnding, MatchedString, Matcher, RejectedString};

pub struct Line;

impl Matcher for Line {
    fn try_match(&self, input: String) -> Result<(MatchedString, LeftoverString), RejectedString> {
        let character = Character;
        let line_ending = LineEnding;

        let mut consumed = String::new();
        let mut unconsumed = input;

        while !unconsumed.is_empty() {
            match line_ending.try_match(unconsumed) {
                Ok((matched_line_ending, leftover_from_line_ending_matcher)) => {
                    consumed = [consumed, matched_line_ending].concat();
                    unconsumed = leftover_from_line_ending_matcher;
                    break;
                }
                Err(rejected_from_line_ending_matcher) => {
                    match character.try_match(rejected_from_line_ending_matcher) {
                        Ok((matched_character, leftover_from_character_matcher)) => {
                            consumed = [consumed, matched_character].concat();
                            unconsumed = leftover_from_character_matcher;
                            continue;
                        }
                        Err(rejected_from_character_matcher) => {
                            unconsumed = rejected_from_character_matcher;
                            break;
                        }
                    }
                }
            }
        }

        Ok((consumed, unconsumed))
    }
}

#[cfg(test)]
mod tests {
    use super::{Line, Matcher};

    #[test]
    fn it_accepts_a_line() {
        let line = Line;

        assert_eq!(
            line.try_match(String::from("line")),
            Ok((String::from("line"), String::new()))
        );

        assert_eq!(
            line.try_match(String::from("line\nanother line")),
            Ok((String::from("line\n"), String::from("another line")))
        );
    }
}
