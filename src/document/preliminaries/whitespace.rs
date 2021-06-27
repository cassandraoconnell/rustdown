use super::{
    LeftoverString, MatchedString, Matcher, RejectedString, SelectionMatcher, CARRIAGE_RETURN,
    FORM_FEED, LINE_TABULATION, NEWLINE, SPACE, TAB,
};

pub struct Whitespace;

impl Matcher for Whitespace {
    fn try_match(&self, input: String) -> Result<(MatchedString, LeftoverString), RejectedString> {
        let matcher = SelectionMatcher::from(vec![
            String::from(CARRIAGE_RETURN),
            String::from(FORM_FEED),
            String::from(LINE_TABULATION),
            String::from(NEWLINE),
            String::from(SPACE),
            String::from(TAB),
        ]);

        matcher.try_match(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Matcher, Whitespace, CARRIAGE_RETURN, FORM_FEED, LINE_TABULATION, NEWLINE, SPACE, TAB,
    };

    #[test]
    fn it_accepts_carriage_return() {
        let whitespace = Whitespace;

        assert_eq!(
            whitespace.try_match(format!(
                "{carriage_return}word",
                carriage_return = CARRIAGE_RETURN
            )),
            Ok((String::from(CARRIAGE_RETURN), String::from("word")))
        );
    }

    #[test]
    fn it_accepts_form_feed() {
        let whitespace = Whitespace;

        assert_eq!(
            whitespace.try_match(format!("{form_feed}word", form_feed = FORM_FEED)),
            Ok((String::from(FORM_FEED), String::from("word")))
        );
    }

    #[test]
    fn it_accepts_line_tabulation() {
        let whitespace = Whitespace;

        assert_eq!(
            whitespace.try_match(format!(
                "{line_tabulation}word",
                line_tabulation = LINE_TABULATION
            )),
            Ok((String::from(LINE_TABULATION), String::from("word")))
        );
    }

    #[test]
    fn it_accepts_newline() {
        let whitespace = Whitespace;

        assert_eq!(
            whitespace.try_match(format!("{newline}word", newline = NEWLINE)),
            Ok((String::from(NEWLINE), String::from("word")))
        );
    }

    #[test]
    fn it_accepts_space() {
        let whitespace = Whitespace;

        assert_eq!(
            whitespace.try_match(format!("{space}word", space = SPACE)),
            Ok((String::from(SPACE), String::from("word")))
        );
    }

    #[test]
    fn it_accepts_tab() {
        let whitespace = Whitespace;

        assert_eq!(
            whitespace.try_match(format!("{tab}word", tab = TAB)),
            Ok((String::from(TAB), String::from("word")))
        );
    }

    #[test]
    fn it_rejects_wrong_characters() {
        let whitespace = Whitespace;

        assert_eq!(
            whitespace.try_match(String::from("not whitespace")),
            Err(String::from("not whitespace"))
        );
    }
}
