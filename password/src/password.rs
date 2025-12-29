pub use argon2::password_hash::Error as Argon2HashError;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub struct ArgonService;
impl ArgonService {
    pub fn hash_password(password: impl Into<String>) -> Result<String, Argon2HashError> {
        let salt: SaltString = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password: String = password.into();
        let hashed_password = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(hashed_password)
    }

    pub fn verify_password(password: impl Into<String>, hash: impl Into<String>) -> bool {
        let hash: String = hash.into();
        let compare_password: String = password.into();
        let Ok(parsed_hash) = PasswordHash::new(&hash) else {
            return false;
        };

        Argon2::default()
            .verify_password(compare_password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
