pub fn md5() {}

pub fn sha1() {}

pub fn sha256() {}

pub fn sha512() {}

pub fn blake3() {}

pub fn escrypt() {}

/// for password
pub fn bcrypt() {}

/// for password
pub fn pbkdf2() {}

/// for password
pub fn scrypt() {}

/// Hashes a password using the Argon2 algorithm (recommended for password hashing).
///
/// This function uses the OWASP5 configuration for Argon2, which is considered
/// secure for password hashing purposes.
///
/// # Parameters
///
/// * `password` - A byte slice containing the password to be hashed.
/// * `salt` - A byte slice containing the salt to be used in the hashing process.
///
/// # Returns
///
/// * `Result<String, argon2::Error>` - A Result containing either:
///   - `Ok(String)`: The hashed password as a String if successful.
///   - `Err(argon2::Error)`: An error if the hashing process fails.
///
/// # Example
///
/// ```
/// let password = b"my_secure_password";
/// let salt = b"random_salt";
/// let hashed_password = argon2(password, salt)?;
/// ```
pub fn argon2(password: &[u8], salt: &[u8]) -> Result<String, argon2::Error> {
  argon2::hash_encoded(password, salt, &argon2::Config::owasp5())
}

pub fn verify_argon2(hash: String, password: &[u8]) -> Result<bool, argon2::Error> {
  argon2::verify_encoded(&hash, password)
}

#[cfg(test)]
mod tests {
  use crate::hash::{argon2, verify_argon2};
  #[test]
  fn test_argon2() {
    let password = b"aurora";
    let hash = argon2(password, b"ohersalt").unwrap();
    let matches = verify_argon2(hash, password).unwrap();
    assert!(matches)
  }
}
