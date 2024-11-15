use rand::{seq::SliceRandom, Rng};

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
  rand::thread_rng().gen_range(min..=max)
}

pub fn min_max_float(min: f64, max: f64) -> f64 {
  rand::thread_rng().gen_range(min..=max)
}

pub fn random_bool() -> bool {
  rand::thread_rng().gen_bool(0.5)
}

pub fn random_choice<T>(slice: &[T]) -> Option<&T> {
  slice.choose(&mut rand::thread_rng())
}

pub fn shuffle<T>(slice: &mut [T]) {
  slice.shuffle(&mut rand::thread_rng())
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
