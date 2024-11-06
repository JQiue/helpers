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
/// for password (recommend)
pub fn argon2(password: &[u8], salt: &[u8]) -> Result<String, argon2::Error> {
  argon2::hash_encoded(password, salt, &argon2::Config::owasp5())
}
/// .
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
