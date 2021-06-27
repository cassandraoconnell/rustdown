use super::{
    LeftoverString, MatchedString, Matcher, RejectedString, SelectionMatcher, CARRIAGE_RETURN,
    NEWLINE,
};

pub struct LineEnding;

impl Matcher for LineEnding {
    fn try_match(&self, input: String) -> Result<(MatchedString, LeftoverString), RejectedString> {
        let matcher = SelectionMatcher::from(vec![
            [String::from(CARRIAGE_RETURN), String::from(NEWLINE)].concat(),
            String::from(CARRIAGE_RETURN),
            String::from(NEWLINE),
        ]);

        matcher.try_match(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{LineEnding, Matcher, CARRIAGE_RETURN, NEWLINE};

    #[test]
    fn it_accepts_newline() {
        let line_ending = LineEnding;

        assert_eq!(
            line_ending.try_match(format!("{newline}another line", newline = NEWLINE)),
            Ok((String::from(NEWLINE), String::from("another line")))
        );
    }

    #[test]
    fn it_accepts_carriage_return_not_followed_by_newline() {
        let line_ending = LineEnding;

        assert_eq!(
            line_ending.try_match(format!(
                "{carriage_return}another line",
                carriage_return = CARRIAGE_RETURN
            )),
            Ok((String::from(CARRIAGE_RETURN), String::from("another line")))
        );
    }

    #[test]
    fn it_accepts_carriage_return_followed_by_newline() {
        let line_ending = LineEnding;

        assert_eq!(
            line_ending.try_match(format!(
                "{carriage_return}{newline}another line",
                carriage_return = CARRIAGE_RETURN,
                newline = NEWLINE
            )),
            Ok((
                format!(
                    "{carriage_return}{newline}",
                    carriage_return = CARRIAGE_RETURN,
                    newline = NEWLINE
                ),
                String::from("another line")
            ))
        );
    }

    #[test]
    fn it_rejects_wrong_characters() {
        let line_ending = LineEnding;

        assert_eq!(
            line_ending.try_match(String::from("not a line ending")),
            Err(String::from("not a line ending"))
        );
    }
}
