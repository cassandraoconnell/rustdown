use super::utils::matcher::{LeftoverString, MatchedString, Matcher, RejectedString};

pub struct SelectionMatcher {
    selection: Vec<String>,
}

impl Matcher for SelectionMatcher {
    fn try_match(&self, input: String) -> Result<(MatchedString, LeftoverString), RejectedString> {
        let mut unconsumed = String::new();

        if let Some(matched_selection) = self.selection.iter().find(|selection| {
            let mut consumed = String::new();
            let mut to_match = input.chars();

            for selection_character in selection.chars() {
                if let Some(character_to_match) = to_match.next() {
                    consumed.push(character_to_match);
                    if character_to_match == selection_character {
                        continue;
                    }

                    break;
                }
            }

            if consumed == **selection {
                unconsumed = to_match.collect();
                return true;
            }

            false
        }) {
            return Ok((String::from(matched_selection), unconsumed));
        }

        Err(input)
    }
}

impl From<Vec<String>> for SelectionMatcher {
    fn from(selection: Vec<String>) -> Self {
        SelectionMatcher { selection }
    }
}

#[cfg(test)]
mod tests {
    use super::{Matcher, SelectionMatcher};

    #[test]
    fn it_accepts_matched_and_returns_leftover() {
        let a_or_b_matcher = SelectionMatcher::from(vec![String::from('a'), String::from('b')]);

        assert_eq!(
            a_or_b_matcher.try_match(String::from('a')),
            Ok((String::from('a'), String::new()))
        );

        assert_eq!(
            a_or_b_matcher.try_match(String::from('b')),
            Ok((String::from('b'), String::new()))
        );

        assert_eq!(
            a_or_b_matcher.try_match(String::from("abc")),
            Ok((String::from('a'), String::from("bc")))
        );

        assert_eq!(
            a_or_b_matcher.try_match(String::from("bc")),
            Ok((String::from('b'), String::from('c')))
        );
    }

    #[test]
    fn it_rejects_mismatched_and_returns_original() {
        let a_or_b_matcher = SelectionMatcher::from(vec![String::from('a'), String::from('b')]);

        assert_eq!(
            a_or_b_matcher.try_match(String::from('c')),
            Err(String::from('c'))
        );

        assert_eq!(a_or_b_matcher.try_match(String::new()), Err(String::new()));
    }
}
