#![allow(dead_code)]

/**
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 * What is the 10 001st prime number?
 */
use crate::problem_3::is_prime;

pub fn nth_prime(n: usize) -> usize {
    (2..)
        .filter(|&val| is_prime(val))
        .skip(n - 1)
        .take(1)
        .collect::<Vec<usize>>()[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_10_001() {
        assert_eq!(nth_prime(10_001), 104_743);
    }

    #[test]
    fn for_100() {
        assert_eq!(nth_prime(100), 541);
    }

    #[test]
    fn for_10() {
        assert_eq!(nth_prime(10), 29);
    }

    #[test]
    fn for_6() {
        assert_eq!(nth_prime(6), 13);
    }
}
