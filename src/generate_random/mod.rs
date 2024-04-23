use rand::{Rng, thread_rng};

pub fn generate_random_int(rng_max: i32) -> i32 {
    let mut rng = thread_rng();
    let random_number = rng.gen_range(1..=rng_max);
    random_number
}



// Generates a random integer between 1 and the given maximum value (inclusive)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_random_integer_within_range() {
        let rng_max = 10;
        let result = generate_random_int(rng_max);
        assert!(result >= 1 && result <= rng_max);
    }

    #[test]
    fn returns_1_when_maximum_is_1() {
        let rng_max = 1;
        let result = generate_random_int(rng_max);
        assert_eq!(result, 1);
    }
}