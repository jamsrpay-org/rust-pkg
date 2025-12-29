use crate::ArgonService;
use rand::Rng;

/// Configuration constants for recovery codes
mod config {
    pub const CODE_COUNT: usize = 8;
    pub const CODE_LENGTH: usize = 12; // xxxx-xxxx-xxxx
    pub const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789"; // 32 chars, no ambiguous
}

/// Represents a recovery code with its hashed version
#[derive(Debug, Clone)]
pub struct RecoveryCode {
    pub plain: String,  // Show to user only once
    pub hashed: String, // Store in database
}

pub struct RecoveryCodeService;

impl RecoveryCodeService {
    /// Generate recovery codes for 2FA setup
    pub fn generate_recovery_codes() -> Result<Vec<RecoveryCode>, argon2::password_hash::Error> {
        let mut rng = rand::rng();
        let mut codes = Vec::with_capacity(config::CODE_COUNT);

        for _ in 0..config::CODE_COUNT {
            let plain = Self::generate_single_code(&mut rng);
            let hashed = ArgonService::hash_password(&plain)?;

            codes.push(RecoveryCode { plain, hashed });
        }

        Ok(codes)
    }

    /// Generate a single recovery code in format xxxx-xxxx-xxxx
    fn generate_single_code(rng: &mut impl Rng) -> String {
        let charset = config::CHARSET;
        let mut code = String::with_capacity(14); // 12 chars + 2 hyphens

        for i in 0..config::CODE_LENGTH {
            let idx = rng.random_range(0..charset.len());
            code.push(charset[idx] as char);

            // Add hyphen after every 4 characters (but not at the end)
            if (i + 1) % 4 == 0 && i != config::CODE_LENGTH - 1 {
                code.push('-');
            }
        }

        code
    }

    /// Verify a recovery code against its stored hash
    pub fn verify_recovery_code(code: &str, stored_hash: &str) -> bool {
        let formatted = Self::format_recovery_code(code);
        ArgonService::verify_password(formatted, stored_hash)
    }

    /// Hash a recovery code using Argon2id
    pub fn hash_recovery_code(code: &str) -> Result<String, argon2::password_hash::Error> {
        ArgonService::hash_password(code)
    }

    /// Normalize recovery code: remove hyphens/spaces, convert to uppercase
    fn normalize_recovery_code(code: &str) -> String {
        code.chars()
            .filter(|c| !c.is_whitespace() && *c != '-')
            .map(|c| c.to_ascii_uppercase())
            .collect()
    }

    /// Format plain code for display with hyphens
    fn format_recovery_code(code: &str) -> String {
        let normalized = Self::normalize_recovery_code(code);
        let chars: Vec<char> = normalized.chars().collect();

        format!(
            "{}-{}-{}",
            chars[0..4].iter().collect::<String>(),
            chars[4..8].iter().collect::<String>(),
            chars[8..12].iter().collect::<String>()
        )
    }
}

#[cfg(test)]
mod recovery_code_tests {
    use super::RecoveryCodeService;
    use super::*;

    #[test]
    fn test_generate_recovery_codes() {
        let codes =
            RecoveryCodeService::generate_recovery_codes().expect("Failed to generate codes");
        dbg!(&codes);
        assert_eq!(codes.len(), config::CODE_COUNT);

        for code in &codes {
            assert_eq!(code.plain.len(), 14);
            assert_eq!(code.plain.chars().filter(|c| *c == '-').count(), 2);
            assert!(!code.hashed.is_empty());
        }
    }

    #[test]
    fn test_verify_recovery_code() {
        let code = "ABCD-1234-EFGH";
        let hashed = RecoveryCodeService::hash_recovery_code(code).expect("Failed to hash");

        assert!(RecoveryCodeService::verify_recovery_code(code, &hashed));
        assert!(RecoveryCodeService::verify_recovery_code(
            "ABCD1234EFGH",
            &hashed
        ));
        assert!(RecoveryCodeService::verify_recovery_code(
            "abcd-1234-efgh",
            &hashed
        ));
        assert!(!RecoveryCodeService::verify_recovery_code(
            "WRONG-CODE-HERE",
            &hashed
        ));
    }

    #[test]
    fn test_normalize_recovery_code() {
        assert_eq!(
            RecoveryCodeService::normalize_recovery_code("ABCD-1234-EFGH"),
            "ABCD1234EFGH"
        );
        assert_eq!(
            RecoveryCodeService::normalize_recovery_code("abcd-1234-efgh"),
            "ABCD1234EFGH"
        );
        assert_eq!(
            RecoveryCodeService::normalize_recovery_code("ABCD 1234 EFGH"),
            "ABCD1234EFGH"
        );
    }
}
