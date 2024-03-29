use serde::{Deserialize, Serialize};
use validator::validate_email;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ValidEmail(String);

impl ValidEmail {
    pub fn parse(s: String) -> Result<ValidEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for ValidEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ValidEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::Gen;

    use super::ValidEmail;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(_g: &mut Gen) -> Self {
            let email = SafeEmail().fake();
            Self(email)
        }
    }

    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email = SafeEmail().fake();
        claim::assert_ok!(ValidEmail::parse(email));
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(ValidEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(ValidEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(ValidEmail::parse(email));
    }

    #[test]
    fn valid_email_as_ref() {
        let email = SafeEmail().fake();
        let valid_email_result = ValidEmail::parse(email);
        claim::assert_ok!(&valid_email_result);
        let valid_email = valid_email_result.unwrap();

        valid_email.as_ref();
    }
}
