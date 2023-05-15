use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_whtspaces = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 255;

        let forbiddens = [
            '/', '(', ')', '"', '<', '>', '\'', '\\', '{', '}', '[', ']', '*', '%', '!',
        ];
        let is_containing_forbiddes = s.chars().any(|c| forbiddens.contains(&c));

        if is_empty_whtspaces || is_too_long || is_containing_forbiddes {
            return Err(format!("{} is not a valid subscriber Name", s));
        }
        // If not panics than the name must be correct
        Ok(Self(s))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ë".repeat(255);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_is_not_valid() {
        let name = "a".repeat(260);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn reject_if_only_whitespaces() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn reject_if_empty() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn name_contains_invalid_char_get_rejected() {
        for name in &[
            '/', '(', ')', '"', '<', '>', '\'', '\\', '{', '}', '[', ']', '*', '%', '!',
        ] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn valid_name_success() {
        let name = "Cristiano Romaldetti".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
