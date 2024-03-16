use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    // Returns an instance of `SubscriberName` if the input satisfies all
    // our validation constraints on subscriber names.
    // It panics otherwise.
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '{', '}', '\\'];
        let contains_forbidden_character = s.chars().any(|e| forbidden_characters.contains(&e));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_character {
            Err(format!("{} is not a valid subscriber name", s))
        } else {
            Ok(Self(s))
        }
    }
    pub fn inner(self) -> String {
        // The caller gets the inner string
        // but they no longer have SubscriberName anymore!
        self.0
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

//---------------TESTS---------------

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_is_rejected() {
        let name = "ë".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn white_space_only_names_are_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_contain_invalid_characters_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '{', '}', '\\'] {
            assert_err!(SubscriberName::parse(name.to_string()));
        }
    }

    #[test]
    fn a_valid_name_is_parsed() {
        let name = "Ursula le guin".into();
        assert_ok!(SubscriberName::parse(name));
    }
}
