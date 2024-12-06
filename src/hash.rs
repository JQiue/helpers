//! Provides a variety of common hash functions

use md5::Digest;

/// Computes the MD5 hash of the input byte slice
///
/// # Parameters
///
/// * `data` - A reference to the byte slice to compute the MD5 hash for
///
/// # Returns
///
/// An MD5 digest of the input data
///
/// # Examples
///
/// ```rust
/// let input = b"hello world";
/// let hash = md5(input);
/// ```
///
/// # Notes
///
/// This function uses the `compute` method from the `md5` crate to calculate the MD5 hash
pub fn md5(data: &[u8]) -> String {
  let mut hasher = md5::Md5::new();
  hasher.update(data);
  base16ct::lower::encode_string(&hasher.finalize())
}

pub fn sha1() {}

pub fn sha256() {}

pub fn sha512() {}

pub fn blake3() {}

pub fn escrypt() {}

/// Generates a bcrypt hash for the given password
///
/// # Parameters
///
/// * `password` - A byte slice containing the password to be hashed
///
/// # Returns
///
/// A `Result` containing the bcrypt hashed password string or a `BcryptError`
///
/// # Examples
///
/// ```rust
/// let password = b"my_secure_password";
/// match bcrypt(password) {
///     Ok(hash) => println!("Hashed password: {}", hash),
///     Err(e) => eprintln!("Hashing failed: {}", e),
/// }
/// ```
///
/// # Notes
///
/// Uses the default bcrypt cost factor for password hashing
pub fn bcrypt(password: &[u8]) -> Result<String, bcrypt::BcryptError> {
  bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

/// Verifies a password against a bcrypt hash
///
/// # Parameters
///
/// * `password` - A byte slice containing the password to verify
/// * `hash` - The bcrypt hash string to compare against
///
/// # Returns
///
/// A `Result` containing a boolean indicating whether the password matches the hash,
/// or a `BcryptError` if verification fails
///
/// # Examples
///
/// ```rust
/// let password = b"my_password";
/// let hash = bcrypt(password).unwrap();
/// match verify_bcrypt(password, hash) {
///     Ok(true) => println!("Password verified successfully"),
///     Ok(false) => println!("Password verification failed"),
///     Err(e) => eprintln!("Verification error: {}", e),
/// }
/// ```
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

/// Verifies a password against an Argon2 encoded hash
///
/// # Parameters
///
/// * `hash` - The Argon2 encoded hash string
/// * `password` - A byte slice containing the password to verify
///
/// # Returns
///
/// A `Result` containing a boolean indicating whether the password matches the hash,
/// or an `Argon2Error` if verification fails
///
/// # Examples
///
/// ```rust
/// let password = b"my_secure_password";
/// let hash = "some_argon2_encoded_hash";
/// match verify_argon2(hash.to_string(), password) {
///     Ok(true) => println!("Password verified successfully"),
///     Ok(false) => println!("Password verification failed"),
///     Err(e) => eprintln!("Verification error: {}", e),
/// }
/// ```
///
/// # Notes
///
/// Uses Argon2's `verify_encoded` method for password verification
pub fn verify_argon2(hash: String, password: &[u8]) -> Result<bool, argon2::Error> {
  argon2::verify_encoded(&hash, password)
}

#[cfg(test)]
mod tests {
  use crate::hash::{argon2, bcrypt, md5, verify_argon2, verify_bcrypt};
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
  #[test]
  fn test_md5() {
    let data = b"hello world";
    let md5_hash = md5(data);
    assert_eq!(md5_hash, md5(data));
  }
}
