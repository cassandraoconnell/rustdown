use super::{IndividualMatcher, LeftoverString, MatchedString, Matcher, RejectedString, SPACE};

pub struct Space;

impl Space {
    pub fn get_literal(&self) -> String {
        String::from(SPACE)
    }
}

impl Matcher for Space {
    fn try_match(&self, input: String) -> Result<(MatchedString, LeftoverString), RejectedString> {
        let matcher = IndividualMatcher::from(self.get_literal());

        matcher.try_match(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{Matcher, Space, SPACE};

    #[test]
    fn it_accepts_space() {
        let space = Space;

        assert_eq!(
            space.try_match(format!("{}word", SPACE)),
            Ok((String::from(SPACE), String::from("word")))
        );
    }

    #[test]
    fn it_rejects_wrong_characters() {
        let space = Space;

        assert_eq!(
            space.try_match(String::from("not a space")),
            Err(String::from("not a space"))
        );
    }
}
