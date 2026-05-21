#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpUrl(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpUrlError {
    InvalidUrl,
    TooLong(usize),
}

impl HttpUrl {
    const MAX_LENGTH: usize = 500;

    pub fn max_length() -> usize {
        Self::MAX_LENGTH
    }

    pub fn new(url: String) -> Result<Self, HttpUrlError> {
        if url.len() > Self::MAX_LENGTH {
            return Err(HttpUrlError::TooLong(url.len()));
        }

        if url::Url::parse(&url).is_err() {
            return Err(HttpUrlError::InvalidUrl);
        }

        Ok(Self(url))
    }

    pub fn from_trusted(url: impl Into<String>) -> Self {
        Self(url.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
