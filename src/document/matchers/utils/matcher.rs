pub type MatchedString = String;
pub type LeftoverString = String;
pub type RejectedString = String;

pub trait Matcher {
    fn try_match(&self, input: String) -> Result<(MatchedString, LeftoverString), RejectedString>;
}
