//! Provides a variety of common hash functions

use md5::Digest;

pub use argon2::Error;
pub use bcrypt::BcryptError;
pub use bcrypt::Version;

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
/// let input = "hello world";
/// let hash = md5(input);
/// ```
///
/// # Notes
///
/// This function uses the `compute` method from the `md5` crate to calculate the MD5 hash
pub fn md5(data: &str) -> String {
  let mut hasher = md5::Md5::new();
  hasher.update(data);
  base16ct::lower::encode_string(&hasher.finalize())
}

pub fn sha1() {}

pub fn sha256() {}

pub fn sha512() {}

/// Compute the BLAKE3 hash of input data
///
/// 计算输入数据的 BLAKE3 哈希值
///
/// BLAKE3 is a modern cryptographic hash function that provides high performance and security
///
/// BLAKE3 是一个现代的加密哈希函数，提供高性能和安全性
///
/// # Parameters / 参数
///
/// * `data` - Input byte slice reference to be hashed / 要计算哈希的输入字节切片引用
///
/// # Returns / 返回值
///
/// Returns the hexadecimal string representation of the BLAKE3 hash
///
/// 返回 BLAKE3 哈希的十六进制字符串表示
///
/// # Examples / 示例
///
/// ```rust
/// let input = b"hello world";
/// let hash = blake3(input);
/// println!("BLAKE3 hash: {}", hash);
/// ```
///
/// # Notes / 注意
///
/// - BLAKE3 is a fast and secure hash algorithm / BLAKE3 是一个快速且安全的哈希算法
/// - The output is a fixed-length 256-bit hash (64-character hexadecimal string) / 输出是定长的 256 位（64个字符的十六进制字符串）
/// - Always produces the same hash for the same input / 对于相同的输入，总是产生相同的哈希值
pub fn blake3(data: &[u8]) -> String {
  let hased = blake3::hash(data);
  hased.to_string()
}

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
/// let password = "my_secure_password";
/// match bcrypt(password) {
///     Ok(hash) => println!("Hashed password: {}", hash),
///     Err(e) => eprintln!("Hashing failed: {}", e),
/// }
/// ```
///
/// # Notes
///
/// Uses the default bcrypt cost factor for password hashing
pub fn bcrypt(password: &str) -> Result<String, bcrypt::BcryptError> {
  bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

/// This function generates a bcrypt hash using a custom cost and version
///
/// 使用自定义成本和版本生成 bcrypt 哈希
///
/// # Parameters / 参数
///
/// * `password` - The password to hash, as a byte slice / 要哈希的密码，作为字节切片
/// * `cost` - The cost parameter for hashing, indicating the computational complexity / 哈希成本参数，表示计算复杂度
/// * `version` - The bcrypt version used to format the hash / bcrypt 版本，用于格式化哈希
///
/// # Returns / 返回值
///
/// Returns a `Result` containing the bcrypt hash as a string if successful, or a `BcryptError` if an error occurs.
///
/// 返回一个包含 bcrypt 哈希的 `Result`，如果成功则包含哈希字符串，否则包含 `BcryptError`
///
/// # Example / 示例
///
/// ```
/// let password = "my_password";
/// let cost = 12;
/// let version = bcrypt::Version::TwoB;
/// let hash = bcrypt_custom(password, cost, version).unwrap();
/// println!("bcrypt hash: {}", hash);
/// ```
pub fn bcrypt_custom(
  password: &str,
  cost: u32,
  version: bcrypt::Version,
) -> Result<String, BcryptError> {
  Ok(bcrypt::hash_with_result(password, cost)?.format_for_version(version))
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
/// let password = "my_password";
/// let hash = bcrypt(password).unwrap();
/// match verify_bcrypt(password, &hash) {
///     Ok(true) => println!("Password verified successfully"),
///     Ok(false) => println!("Password verification failed"),
///     Err(e) => eprintln!("Verification error: {}", e),
/// }
/// ```
pub fn verify_bcrypt(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
  bcrypt::verify(password, hash)
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
/// let password = "my_secure_password";
/// let salt = "random_salt";
/// let hashed_password = argon2(password, salt)?;
/// ```
pub fn argon2(password: &str, salt: &str) -> Result<String, argon2::Error> {
  argon2::hash_encoded(
    password.as_bytes(),
    salt.as_bytes(),
    &argon2::Config::owasp5(),
  )
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
/// let password = "my_secure_password";
/// let hash = "some_argon2_encoded_hash";
/// match verify_argon2(hash, password) {
///     Ok(true) => println!("Password verified successfully"),
///     Ok(false) => println!("Password verification failed"),
///     Err(e) => eprintln!("Verification error: {}", e),
/// }
/// ```
///
/// # Notes
///
/// Uses Argon2's `verify_encoded` method for password verification
pub fn verify_argon2(hash: &str, password: &str) -> Result<bool, argon2::Error> {
  argon2::verify_encoded(hash, password.as_bytes())
}

#[cfg(test)]
mod tests {
  use crate::hash::{argon2, bcrypt, bcrypt_custom, blake3, md5, verify_argon2, verify_bcrypt};
  #[test]
  fn test_argon2() {
    let password = "my_secure_password";
    let hash = argon2(password, "ohersalt").unwrap();
    assert!(verify_argon2(&hash, password).unwrap())
  }
  #[test]
  fn test_bcrypt() {
    let password = "my_secure_password";
    let hashed_password = bcrypt(password).unwrap();
    assert!(verify_bcrypt(password, &hashed_password).unwrap());
  }
  #[test]
  fn test_bcrypt_custom() {
    let password = "my_secure_password";
    let hashed_password = bcrypt_custom(password, 8, bcrypt::Version::TwoA).unwrap();
    assert!(verify_bcrypt(password, &hashed_password).unwrap());
  }
  #[test]
  fn test_md5() {
    let data = "hello world";
    assert_eq!(md5(data), md5(data));
  }
  #[test]
  fn test_blake3() {
    let data = b"hello world";
    let hased = blake3(data);
    assert_eq!(hased, blake3(data));
  }
}
