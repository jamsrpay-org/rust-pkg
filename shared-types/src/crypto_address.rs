#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoAddress(String);

impl std::fmt::Display for CryptoAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl CryptoAddress {
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }

    pub fn from_trusted(address: impl Into<String>) -> Self {
        Self(address.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
