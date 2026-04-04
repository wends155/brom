use crate::error::AuthError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;

/// Hashes a password using Argon2id with default parameters.
///
/// # Errors
/// Returns `AuthError::HashError` if hashing fails.
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AuthError::HashError(e.to_string()))?
        .to_string();

    Ok(password_hash)
}

/// Verifies a password against an Argon2id hash.
///
/// # Errors
/// Returns `AuthError::InvalidCredentials` if verification fails.
/// Returns `AuthError::HashError` if the hash is malformed.
pub fn verify_password(password: &str, hash: &str) -> Result<(), AuthError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| AuthError::HashError(e.to_string()))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AuthError::InvalidCredentials)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "my_secure_password";
        let hash = hash_password(password).expect("Failed to hash password");

        // Should verify correctly
        assert!(verify_password(password, &hash).is_ok());

        // Should fail with wrong password
        assert!(matches!(
            verify_password("wrong_password", &hash),
            Err(AuthError::InvalidCredentials)
        ));
    }

    #[test]
    fn test_invalid_hash_format() {
        let result = verify_password("password", "not_a_valid_hash");
        assert!(matches!(result, Err(AuthError::HashError(_))));
    }
}
