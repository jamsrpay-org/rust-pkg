use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct DisplayName(String);

#[derive(Debug, Error)]
pub enum DisplayNameError {
    #[error("Display name is too long. Maximum length is 100 characters")]
    TooLong,
}

impl DisplayName {
    pub fn new(value: impl Into<String>) -> Result<Self, DisplayNameError> {
        let value = value.into().trim().to_string();

        if value.len() > 100 {
            return Err(DisplayNameError::TooLong);
        }

        Ok(Self(value))
    }

    /// Reconstruct from persistence (no validation).
    pub fn from_trusted(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for DisplayName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
