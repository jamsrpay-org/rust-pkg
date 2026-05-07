use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageUrlError {
    #[error("Invalid logo url")]
    InvalidUrl,
    #[error("Logo url is too long. Maximum length is 500 characters")]
    TooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageUrl(String);

impl ImageUrl {
    pub fn new(logo: String) -> Result<Self, ImageUrlError> {
        if logo.is_empty() {
            return Err(ImageUrlError::InvalidUrl);
        }

        if logo.len() > 500 {
            return Err(ImageUrlError::TooLong);
        }

        if url::Url::parse(&logo).is_err() {
            return Err(ImageUrlError::InvalidUrl);
        }

        Ok(Self(logo))
    }

    pub fn from_trusted(logo: impl Into<String>) -> Self {
        Self(logo.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
