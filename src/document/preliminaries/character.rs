use super::{LeftoverString, MatchedString, Matcher, RejectedString};

pub struct Character;

impl Matcher for Character {
    fn try_match(&self, input: String) -> Result<(MatchedString, LeftoverString), RejectedString> {
        let mut characters = input.chars();

        if let Some(character_to_match) = characters.next() {
            return Ok((String::from(character_to_match), characters.collect()));
        }

        Err(characters.collect())
    }
}

#[cfg(test)]
mod tests {
    use super::{Character, Matcher};

    #[test]
    fn it_accepts_any_character() {
        let character = Character;

        assert_eq!(
            character.try_match(String::from("ab")),
            Ok((String::from("a"), String::from("b")))
        );
    }

    #[test]
    fn it_rejects_empty_string() {
        let character = Character;

        assert_eq!(character.try_match(String::new()), Err(String::new()));
    }
}
