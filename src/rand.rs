use rand::{
  seq::{IndexedRandom, SliceRandom},
  Rng,
};

/// Generates a random integer within the specified range (inclusive).
///
/// # Parameters
///
/// * `min`: The minimum value (inclusive) for the generated random number.
/// * `max`: The maximum value (inclusive) for the generated random number.
///
/// # Returns
///
/// An integer randomly selected within the range from `min` (inclusive) to `max` (inclusive).
///
/// # Example
///
/// ```rust
/// use helpers::rand::min_max;
///
/// let random_number = min_max(1, 10);
/// println!("{}", random_number); // Prints a random number between 1 and 10 (inclusive)
/// ```
pub fn min_max(min: i32, max: i32) -> i32 {
  rand::rng().random_range(min..=max)
}

/// Generate a random floating-point number within a specified range
///
/// # Parameters
///
/// - `min`: The minimum value of the range (inclusive)
/// - `max`: The maximum value of the range (inclusive)
///
/// # Returns
///
/// A random `f64` value between `min` and `max`
///
/// # Examples
///
/// ```rust
/// let random_val = min_max_float(0.0, 10.0);
/// println!("Random float between 0 and 10: {}", random_val);
/// ```
///
/// # Panics
///
/// - Panics if `min` is greater than `max`
/// - Uses thread-local random number generator
pub fn min_max_float(min: f64, max: f64) -> f64 {
  rand::rng().random_range(min..=max)
}

/// Generate a random boolean value
///
/// # Returns
///
/// A random `bool` with a 50% probability of being true or false
///
/// # Examples
///
/// ```rust
/// let coin_flip = random_bool();
/// println!("Random boolean: {}", coin_flip);
/// ```
///
/// # Notes
///
/// - Uses a thread-local random number generator
/// - Probability of `true` is exactly 0.5
pub fn random_bool() -> bool {
  rand::rng().random_bool(0.5)
}

/// Randomly select an element from a slice
///
/// # Type Parameters
///
/// - `T`: The type of elements in the slice
///
/// # Parameters
///
/// - `slice`: A reference to a slice of elements
///
/// # Returns
///
/// - `Some(&T)` containing a randomly chosen reference to an element
/// - `None` if the slice is empty
///
/// # Examples
///
/// ```rust
/// let fruits = vec!["apple", "banana", "cherry"];
/// if let Some(fruit) = random_choice(&fruits) {
///     println!("Randomly selected fruit: {}", fruit);
/// }
/// ```
///
/// # Notes
///
/// - Uses thread-local random number generator
/// - Returns `None` for empty slices
pub fn random_choice<T>(slice: &[T]) -> Option<&T> {
  slice.choose(&mut rand::rng())
}

/// Randomly shuffle the elements of a mutable slice in-place
///
/// # Type Parameters
///
/// - `T`: The type of elements in the slice
///
/// # Parameters
///
/// - `slice`: A mutable reference to a slice of elements
///
/// # Examples
///
/// ```rust
/// let mut numbers = vec![1, 2, 3, 4, 5];
/// shuffle(&mut numbers);
/// println!("Shuffled numbers: {:?}", numbers);
/// ```
///
/// # Notes
///
/// - Modifies the slice in-place
/// - Uses the Fisher-Yates (Knuth) shuffle algorithm
/// - Uses thread-local random number generator
pub fn shuffle<T>(slice: &mut [T]) {
  slice.shuffle(&mut rand::rng())
}

#[cfg(test)]
mod tests {
  use crate::rand::{min_max, min_max_float, random_bool, random_choice, shuffle};
  #[test]
  fn it_works() {
    println!("{}", min_max(1, 10));
    println!("{}", min_max_float(1.0, 10.0));
    println!("{}", random_bool());
    if let Some(choice) = random_choice(&vec![1, 2, 3, 4, 5]) {
      println!("Random choice: {}", choice);
    }
    let mut nums = vec![1, 2, 3, 4, 5];
    shuffle(&mut nums);
    println!("Shuffled array: {:?}", nums);
  }
}
