use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub use jsonwebtoken::errors::{Error, ErrorKind};

pub mod jwt_numeric_date {
  //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
  use chrono::{DateTime, TimeZone, Utc};
  use serde::{self, Deserialize, Deserializer, Serializer};
  /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
  pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_i64(date.timestamp())
  }

  /// Attempts to deserialize an i64 and use as a Unix timestamp
  pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let timestamp = i64::deserialize(deserializer)?;
    Utc
      .timestamp_opt(timestamp, 0)
      .single()
      .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Claims<T>
where
  T: Serialize,
{
  pub data: T,
  #[serde(with = "jwt_numeric_date")]
  pub iat: DateTime<Utc>,
  #[serde(with = "jwt_numeric_date")]
  pub exp: DateTime<Utc>,
}

impl<T> Claims<T>
where
  T: Serialize + DeserializeOwned + 'static,
{
  /// .
  pub fn new(data: T, exp: i64) -> Self {
    let iat = Utc::now();
    let exp = iat + Duration::seconds(exp);
    Self { data, iat, exp }
  }
}

/// Creates a JWT (JSON Web Token) from the provided data and key, with an expiration time specified in seconds.
///
/// # Parameters
///
/// * `data` - The data to be included in the JWT claims. It must implement the `Serialize` and `DeserializeOwned` traits.
/// * `key` - The secret key used to sign the JWT. It should be a string.
/// * `expire` - The expiration time of the JWT in seconds from the current time.
///
/// # Returns
///
/// * `Result<String, jsonwebtoken::errors::Error>` - On success, returns a `Result` containing the JWT as a string.
///   On error, returns a `Result` containing a `jsonwebtoken::errors::Error`.
///
/// # Example
///
/// ```rust
/// use serde::{Deserialize, Serialize};
/// use helpers::jwt::{sign, Claims};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct Data {
///     user_id: String,
///     is_plus: i8,
/// }
///
/// let key = "test_key".to_string();
/// let data = Data {
///     user_id: "id_001".to_string(),
///     is_plus: 1,
/// };
/// let token = sign::<Data>(data, key.clone(), 7).unwrap();
/// println!("{token:?}");
/// ```
pub fn sign<T>(
  data: T,
  key: String,
  expire: i64,
) -> Result<std::string::String, jsonwebtoken::errors::Error>
where
  T: Serialize + DeserializeOwned + 'static,
{
  let claims: Claims<T> = Claims::new(data, expire);
  let key = EncodingKey::from_secret(key.as_ref());
  encode(&Header::default(), &claims, &key)
}

/// Verifies and decodes a JWT (JSON Web Token) using the provided token and key.
///
/// # Parameters
///
/// * `token` - The JWT string to be verified and decoded.
/// * `key` - The secret key used to verify the JWT's signature.
///
/// # Returns
///
/// * `Result<jsonwebtoken::TokenData<Claims<T>>, jsonwebtoken::errors::Error>` - On success, returns a `Result`
///   containing the decoded token data wrapped in `jsonwebtoken::TokenData`. The `Claims<T>` struct contains
///   the custom data, issued at time, and expiration time. On error, returns a `Result` containing a
///   `jsonwebtoken::errors::Error`.
///
/// # Type Parameters
///
/// * `T` - The type of the custom data stored in the JWT claims. It must implement both `Serialize` and `DeserializeOwned` traits.
pub fn verify<T>(
  token: String,
  key: String,
) -> Result<jsonwebtoken::TokenData<Claims<T>>, jsonwebtoken::errors::Error>
where
  T: Serialize + DeserializeOwned,
{
  let key = DecodingKey::from_secret(key.as_ref());
  decode(&token, &key, &Validation::new(Algorithm::HS256))
}

#[cfg(test)]
mod tests {
  use crate::jwt::{sign, verify};
  use serde::{Deserialize, Serialize};
  #[derive(Debug, Serialize, Deserialize)]
  struct Data {
    user_id: String,
    is_plus: i8,
  }
  #[test]
  fn it_works() {
    let key = "test_key".to_string();
    let data = Data {
      user_id: "id_001".to_string(),
      is_plus: 1,
    };
    let token = sign::<Data>(data, key.clone(), 7).unwrap();
    println!("{token:?}");
    let matches = verify::<Data>(token.clone(), key);
    let data = matches.unwrap();
    println!("{:?}", data.claims);
  }
}
