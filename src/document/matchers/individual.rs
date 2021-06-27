use super::utils::matcher::{LeftoverString, MatchedString, Matcher, RejectedString};

pub struct IndividualMatcher {
    individual: String,
}

impl Matcher for IndividualMatcher {
    fn try_match(&self, input: String) -> Result<(MatchedString, LeftoverString), RejectedString> {
        let mut consumed = String::new();
        let mut to_match = input.chars();

        for character in self.individual.chars() {
            if let Some(character_to_match) = to_match.next() {
                consumed.push(character_to_match);
                if character_to_match == character {
                    continue;
                }

                break;
            }
        }

        if consumed == self.individual {
            return Ok((consumed, to_match.collect()));
        }

        Err([consumed, to_match.collect()].concat())
    }
}

impl From<char> for IndividualMatcher {
    fn from(character: char) -> Self {
        IndividualMatcher {
            individual: String::from(character),
        }
    }
}

impl From<String> for IndividualMatcher {
    fn from(string: String) -> Self {
        IndividualMatcher { individual: string }
    }
}

#[cfg(test)]
mod tests {
    use super::{IndividualMatcher, Matcher};

    #[test]
    fn it_accepts_matched_and_returns_leftover() {
        let a_matcher = IndividualMatcher::from(String::from('a'));

        assert_eq!(
            a_matcher.try_match(String::from("a")),
            Ok((String::from("a"), String::new()))
        );

        assert_eq!(
            a_matcher.try_match(String::from("abc")),
            Ok((String::from("a"), String::from("bc")))
        );
    }

    #[test]
    fn it_rejects_mismatched_and_returns_original() {
        let a_matcher = IndividualMatcher::from(String::from('a'));

        assert_eq!(
            a_matcher.try_match(String::from("bc")),
            Err(String::from("bc"))
        );

        assert_eq!(a_matcher.try_match(String::new()), Err(String::new()));
    }
}
