#[cfg(feature = "rand")]
pub mod rand {
    use rand::Rng;

    pub fn generate_random_between_limits(min: i32, max: i32) -> i32 {
        rand::thread_rng().gen_range(min..=max)
    }

    #[cfg(test)]
    mod tests {
        use crate::rand::generate_random_between_limits;
        #[test]
        fn it_works() {
            println!("{}", generate_random_between_limits(1, 10));
        }
    }
}

#[cfg(feature = "jwt")]
pub mod jwt {}
