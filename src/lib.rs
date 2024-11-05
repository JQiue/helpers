#[cfg(feature = "rand")]
pub mod rand {
  use rand::Rng;

  pub fn min_max(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
  }

  #[cfg(test)]
  mod tests {
    use crate::rand::min_max;
    #[test]
    fn it_works() {
      println!("{}", min_max(1, 10));
    }
  }
}

#[cfg(feature = "jwt")]
pub mod jwt {
  use jsonwebtoken::errors::ErrorKind;
  use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
  use serde::{Deserialize, Serialize};
  use time::{Duration, OffsetDateTime};

  pub mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let timestamp = date.unix_timestamp();
      serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
      D: Deserializer<'de>,
    {
      OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
        .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
  }

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  pub struct Claims<T> {
    data: T,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
  }

  // impl<T: Serialize> Serialize for Claims<T> {
  //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  //     where
  //         S: serde::Serializer,
  //     {
  //     }
  // }

  impl<T> Claims<T> {
    pub fn new(data: T, iat: OffsetDateTime, exp: OffsetDateTime) -> Self {
      // normalize the timestamps by stripping of microseconds
      let iat = iat
        .date()
        .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
        .unwrap()
        .assume_utc();
      let exp = exp
        .date()
        .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
        .unwrap()
        .assume_utc();

      Self { data, iat, exp }
    }
  }

  pub fn sign<T>(data: T, key: String, expire: i64) -> String {
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::days(expire);
    let claims: Claims<T> = Claims::new(data, iat, exp);
    let header = Header::default();
    let key = EncodingKey::from_secret(key.as_ref());
    match encode(&header, &claims, &key) {
      Ok(token) => token,
      Err(error) => panic!("{error}"),
    }
  }

  pub fn verify<T>(token: String, key: String) -> jsonwebtoken::TokenData<Claims<T>> {
    let key = DecodingKey::from_secret(key.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    match decode::<Claims<T>>(&token, &key, &validation) {
      Ok(c) => c,
      Err(err) => match *err.kind() {
        ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
        ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
        _ => panic!("Some other errors"),
      },
    }
  }

  #[cfg(test)]
  mod tests {
    use crate::jwt::{sign, Data};
    #[test]
    fn it_works() {
      let key = "aurora";

      let data = Data {
        user_id: "auroraid_8fcgu8hr".to_string(),
        is_plus: 1,
      };
      let token = sign::<Data>(data, key.to_string(), 7);
      println!("{token}")
    }
  }
}

#[cfg(feature = "hash")]
pub mod hash {
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
}

#[cfg(feature = "uuid")]
pub mod uuid {}

#[cfg(feature = "time")]
pub mod time {}
