pub struct Alphabet;

impl Alphabet {
  /// pure number 0-9
  pub const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

  /// lowercase a-z
  pub const LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
  ];

  /// uppercase A-Z
  pub const UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  ];

  /// safety character. Does not contain confusing characters
  pub const SAFE: [char; 32] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M',
    'N', 'P', 'Q', 'R', 'S', 'T', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c',
  ];

  /// default alphabet. Contains letters and numbers
  pub const DEFAULT: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
  ];
}

/// Generates a Universally Unique Identifier (UUID) using a custom alphabet and length.
///
/// This function uses the `nanoid` crate to generate a UUID. The `alphabet` parameter specifies the
/// characters to be used in the UUID. The `length` parameter determines the number of characters in the
/// generated UUID.
///
/// # Parameters
///
/// * `alphabet`: A slice of characters representing the custom alphabet to be used in the UUID.
/// * `length`: An unsigned integer representing the desired length of the generated UUID.
///
/// # Returns
///
/// This function returns a `String` containing the generated UUID.
///
/// # Examples
///
/// ```rust
/// use helpers::uuid::{Alphabet, uuid};
///
/// // Generate a UUID using the default alphabet and length 8
/// let default_uuid = uuid(&Alphabet::DEFAULT, 8);
/// println!("Default UUID: {}", default_uuid);
///
/// // Generate a UUID using only lowercase letters and length 10
/// let lowercase_uuid = uuid(&Alphabet::LOWER, 10);
/// println!("Lowercase UUID: {}", lowercase_uuid);
/// ```
pub fn uuid(alphabet: &[char], length: usize) -> String {
  nanoid::nanoid!(length, alphabet)
}

#[cfg(test)]
mod tests {
  use super::uuid;
  use crate::uuid::Alphabet;

  #[test]
  fn test_uuid() {
    // test pure number
    let numbers = uuid(&Alphabet::NUMBERS, 8);
    println!("Numbers only: {}", numbers);

    // test lowercase
    let lower = uuid(&Alphabet::LOWER, 8);
    println!("Lowercase only: {}", lower);

    // test uppercase
    let upper = uuid(&Alphabet::UPPER, 8);
    println!("Uppercase only: {}", upper);

    // test safety character
    let safe = uuid(&Alphabet::SAFE, 8);
    println!("Safe chars: {}", safe);

    // test the default character set
    let default = uuid(&Alphabet::DEFAULT, 8);
    println!("Default chars: {}", default);
  }
}
