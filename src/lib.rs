pub mod rand {
    use rand::Rng;
    use std::ops::Range;
    pub fn generate_random_between_limits(range: Range<i32>) -> i32 {
        rand::thread_rng().gen_range(range)
    }
}

#[cfg(test)]
mod tests {
    use crate::rand::generate_random_between_limits;
    #[test]
    fn it_works() {
        println!("{}", generate_random_between_limits(1..10));
    }
}
