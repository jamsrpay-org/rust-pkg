use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailAddressError {
    #[error("Invalid email address: {0}")]
    InvalidEmailAddress(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAddress(String);

impl EmailAddress {
    const MAX_LENGTH: usize = 254;

    pub fn new(value: String) -> Result<Self, EmailAddressError> {
        if value.is_empty() || !value.contains('@') || value.len() > Self::MAX_LENGTH {
            return Err(EmailAddressError::InvalidEmailAddress(value));
        }
        Ok(Self(value))
    }

    /// Create from a trusted source (e.g. database) without validation.
    pub fn from_trusted(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for EmailAddress {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = EmailAddress::new("user@example.com".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().value(), "user@example.com");
    }

    #[test]
    fn test_empty_email() {
        let email = EmailAddress::new("".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn test_missing_at_sign() {
        let email = EmailAddress::new("invalid-email".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn test_too_long_email() {
        let email = EmailAddress::new(format!("{}@example.com", "a".repeat(250)));
        assert!(email.is_err());
    }

    #[test]
    fn test_from_trusted_skips_validation() {
        let email = EmailAddress::from_trusted("anything");
        assert_eq!(email.value(), "anything");
    }
}
