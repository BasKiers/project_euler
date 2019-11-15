#![allow(dead_code)]

/**
 * The sum of the squares of the first ten natural numbers is,
 * 12 + 22 + ... + 102 = 385
 *
 * The square of the sum of the first ten natural numbers is,
 * (1 + 2 + ... + 10)2 = 552 = 3025
 *
 * Hence the difference between the sum of the squares of the first ten natural numbers
 * and the square of the sum is 3025 − 385 = 2640.
 *
 * Find the difference between the sum of the squares of the first one hundred natural numbers
 * and the square of the sum.
 */

pub fn problem(to_value: usize) -> usize {
    let (sum, sum_of_squares) =
        (1..=to_value).fold((0, 0), |acc, value| (acc.0 + value, acc.1 + value.pow(2)));

    sum.pow(2) - sum_of_squares
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_100() {
        assert_eq!(problem(100), 25_164_150);
    }

    #[test]
    fn for_10() {
        assert_eq!(problem(10), 2640);
    }
}
