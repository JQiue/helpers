use uuid::Uuid;

pub use uuid::Error;

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

  /// numbers + lowercase
  pub const NUMBERS_LOWER: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
  ];

  /// numbers + uppercase
  pub const NUMBERS_UPPER: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  ];

  /// numbers + lowercase + uppercase
  pub const NUMBERS_LOWER_UPPER: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
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

/// Generates a NanoID with the given alphabet and length.
///
/// # Arguments
///
/// * `alphabet` - The alphabet to use for generating the NanoID.
/// * `length` - The length of the NanoID to generate.
///
/// # Returns
///
/// A NanoID string.
///
/// # Example
///
/// ```
/// use helpers::uuid::{nanoid, Alphabet};
///
/// let id = nanoid(&Alphabet::NUMBERS, 8);
/// println!("Generated NanoID: {}", id);
/// ```
pub fn nanoid(alphabet: &[char], length: usize) -> String {
  nanoid::nanoid!(length, alphabet)
}

/// Generates a segmented NanoID with the given alphabet, length, separator, and number of segments.
///
/// # Arguments
///
/// * `alphabet` - The alphabet to use for generating the NanoID.
/// * `length` - The length of the NanoID to generate.
/// * `separator` - The separator to use between segments.
/// * `segments` - The number of segments to generate.
///
/// # Returns
///
/// A segmented NanoID string.
///
/// # Example
///
/// ```
/// use helpers::uuid::{nanoid_segmented, Alphabet};
///
/// let id = nanoid_segmented(&Alphabet::DEFAULT, 15, '-', 3);
/// println!("Generated segmented NanoID: {}", id);
/// ```
pub fn nanoid_segmented(
  alphabet: &[char],
  length: usize,
  separator: char,
  segments: usize,
) -> String {
  let segment_length = length / segments;
  let remainder = length % segments;
  let mut segments_vec = vec![];
  for i in 0..segments {
    let current_segment_length = if i == segments - 1 {
      segment_length + remainder
    } else {
      segment_length
    };
    let segment = nanoid::nanoid!(current_segment_length, alphabet);
    segments_vec.push(segment);
  }
  segments_vec.join(&separator.to_string())
}

/// Generates a UUID v4.
///
/// # Returns
///
/// A UUID v4.
///
/// # Example
///
/// ```
/// use helpers::uuid::uuid_v4;
///
/// let uuid = uuid_v4();
/// println!("Generated UUID v4: {}", uuid);
/// ```
pub fn uuid_v4() -> Uuid {
  Uuid::new_v4()
}

/// Generates a UUID v5 with the given namespace and name.
///
/// # Arguments
///
/// * `name` - The name to use for generating the UUID v5.
///
/// # Returns
///
/// A UUID v5.
///
/// # Example
///
/// ```
/// use helpers::uuid::uuid_v5;
/// use uuid::Uuid;
///
/// let name = b"example.com";
/// let uuid = uuid_v5(name);
/// println!("Generated UUID v5: {}", uuid);
/// ```
pub fn uuid_v5(name: &[u8]) -> Uuid {
  Uuid::new_v5(&Uuid::NAMESPACE_DNS, name)
}

#[cfg(test)]
mod tests {
  use crate::uuid::{nanoid, nanoid_segmented, uuid_v4, uuid_v5, Alphabet};

  #[test]
  fn test_nanoid() {
    // test pure number
    let numbers = nanoid(&Alphabet::NUMBERS, 8);
    println!("Numbers only: {}", numbers);

    // test lowercase
    let lower = nanoid(&Alphabet::LOWER, 8);
    println!("Lowercase only: {}", lower);

    // test uppercase
    let upper = nanoid(&Alphabet::UPPER, 8);
    println!("Uppercase only: {}", upper);

    // numbers + lowercase
    let numbers_lower = nanoid(&Alphabet::NUMBERS_LOWER, 8);
    println!("Numbers and Lowercase: {}", numbers_lower);

    // test safety character
    let safe = nanoid(&Alphabet::SAFE, 8);
    println!("Safe chars: {}", safe);

    // test the default character set
    let default = nanoid(&Alphabet::DEFAULT, 8);
    println!("Default chars: {}", default);

    // test the delimited UUID
    let segmented1 = nanoid_segmented(&Alphabet::DEFAULT, 15, '-', 3);
    println!("Segmented UUID (3 parts): {}", segmented1);

    let segmented2 = nanoid_segmented(&Alphabet::SAFE, 16, '~', 5);
    println!("Segmented UUID (5 parts): {}", segmented2);
  }

  #[test]
  fn test_uuid_v4() {
    let uuid = uuid_v4();
    assert_eq!(Some(uuid::Version::Random), uuid.get_version());
    println!("{}", uuid);
  }

  #[test]
  fn test_uuid_v5() {
    let name = b"example.com";
    let uuid = uuid_v5(name);
    assert_eq!(uuid_v5(name), uuid);
  }
}
