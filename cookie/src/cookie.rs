pub use cookie::Cookie;
use cookie::SameSite;
use url::Url;

pub struct CustomCookie {
    name: String,
    value: String,
    max_age: u64,
}

impl CustomCookie {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            max_age: 60 * 60 * 24,
        }
    }

    pub fn add_suffix(mut self, env: &str) -> Self {
        self.name = format!("{}-{}", self.name, env);
        self
    }

    pub fn set_max_age(mut self, max_age: u64) -> Self {
        self.max_age = max_age;
        self
    }

    pub fn build(self, app_server_url: &str) -> Cookie<'static> {
        let parsed_url = Url::parse(app_server_url.as_ref());
        // extract domain to save cookie
        // jamsrworld.com -> .jamsrworld.com
        // jamsrpay.jamsrworld.com -> .jamsrworld.com
        let domain = (match parsed_url {
            Ok(url) => url.host_str().and_then(|host| {
                let parts: Vec<&str> = host.split(".").collect();
                if parts.len() > 1 {
                    Some(format!(
                        ".{}.{}",
                        parts[parts.len() - 2],
                        parts[parts.len() - 1]
                    ))
                } else {
                    None
                }
            }),
            Err(_) => None,
        })
        .unwrap_or("".to_string());
        Cookie::build((self.name, self.value))
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::None)
            .domain(domain)
            .max_age(time::Duration::new(self.max_age as i64, 0))
            .build()
    }
}
