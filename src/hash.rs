//! Provides a variety of common hash functions

/// Computes the MD5 hash of input data.
///
/// This function calculates the MD5 (Message Digest algorithm 5) hash,
/// which produces a 128-bit (16-byte) hash value. Note that MD5 is
/// considered cryptographically broken and should not be used for
/// security-critical applications.
pub fn md5() {}

pub fn sha1() {}

pub fn sha256() {}

pub fn sha512() {}

pub fn blake3() {}

pub fn escrypt() {}

/// for password
pub fn bcrypt(password: &[u8]) -> Result<String, bcrypt::BcryptError> {
  bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_bcrypt(password: &[u8], hash: String) -> Result<bool, bcrypt::BcryptError> {
  bcrypt::verify(password, &hash)
}

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
  use crate::hash::{argon2, bcrypt, verify_argon2, verify_bcrypt};
  #[test]
  fn test_argon2() {
    let password = b"my_secure_password";
    let hash = argon2(password, b"ohersalt").unwrap();
    assert!(verify_argon2(hash, password).unwrap())
  }
  #[test]
  fn test_bcrypt() {
    let password = b"my_secure_password";
    let hashed_password = bcrypt(password).unwrap();
    assert!(verify_bcrypt(password, hashed_password).unwrap());
  }
}
