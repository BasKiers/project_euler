#![allow(dead_code)]

/**
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ?
 */

pub fn is_prime(value: usize) -> bool {
    if value <= 3 {
        value > 1
    } else if value % 2 == 0 || value % 3 == 0 {
        false
    } else {
        let mut i = 5;
        while i * i <= value {
            if value % i == 0 || value % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}

pub fn problem(value: usize) -> usize {
    if is_prime(value) {
        return value;
    }

    let first_prime_factor = (2..=(value / 2))
        .filter(|&val| is_prime(val))
        .find(|prime| {
            let factor = value / prime;
            factor * prime == value
        })
        .unwrap();

    problem(value / first_prime_factor)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_600_851_475_143() {
        assert_eq!(problem(600_851_475_143), 6857);
    }

    #[test]
    fn for_34_866() {
        assert_eq!(problem(34_866), 149);
    }

    #[test]
    fn for_13_195() {
        assert_eq!(problem(13_195), 29);
    }

    #[test]
    fn for_42() {
        assert_eq!(problem(42), 7);
    }

    #[test]
    fn for_7() {
        assert_eq!(problem(7), 7);
    }

    #[test]
    fn for_4() {
        assert_eq!(problem(4), 2);
    }

    #[test]
    fn for_2() {
        assert_eq!(problem(2), 2);
    }
}
