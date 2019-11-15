#![allow(dead_code)]

/**
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
 * The sum of these multiples is 23.
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */

pub fn problem(size: usize) -> usize {
    (0..size).fold(0, |acc, x| {
        if x % 3 == 0 || x % 5 == 0 {
            acc + x
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_1000() {
        assert_eq!(problem(1000), 233_168);
    }

    #[test]
    fn for_2() {
        assert_eq!(problem(2), 0);
    }

    #[test]
    fn for_0() {
        assert_eq!(problem(0), 0);
    }
}
