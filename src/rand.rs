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
