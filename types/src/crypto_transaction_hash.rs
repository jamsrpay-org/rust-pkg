#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoTransactionHash(String);

impl std::fmt::Display for CryptoTransactionHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl CryptoTransactionHash {
    pub fn new(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    pub fn from_trusted(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::ops::Deref for CryptoTransactionHash {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for CryptoTransactionHash {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for CryptoTransactionHash {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for CryptoTransactionHash {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl PartialEq<str> for CryptoTransactionHash {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for CryptoTransactionHash {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<CryptoTransactionHash> for &str {
    fn eq(&self, other: &CryptoTransactionHash) -> bool {
        *self == other.0
    }
}

impl PartialEq<CryptoTransactionHash> for str {
    fn eq(&self, other: &CryptoTransactionHash) -> bool {
        self == other.0
    }
}
