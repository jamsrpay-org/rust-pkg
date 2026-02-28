use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CookieParser {
    cookies: HashMap<String, String>,
}

impl CookieParser {
    /// Create a new parser from a raw cookie header string
    pub fn new(raw: &str) -> Self {
        let mut cookies = HashMap::new();

        for pair in raw.split(';') {
            let trimmed = pair.trim();

            if let Some((key, value)) = trimmed.split_once('=') {
                cookies.insert(key.to_string(), value.to_string());
            }
        }

        CookieParser { cookies }
    }

    /// Get a cookie by name
    pub fn get(&self, name: &str) -> Option<&str> {
        self.cookies.get(name).map(|s| s.as_str())
    }
}
