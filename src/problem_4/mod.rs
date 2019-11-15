#![allow(dead_code)]

/**
 * A palindromic number reads the same both ways.
 * The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
 */

pub fn is_palindrome_number(value: i64) -> bool {
    let mut number_string = (value as i64).to_string();

    while number_string.len() > 1 {
        if number_string.pop().unwrap() != number_string.remove(0) {
            return false;
        }
    }
    true
}

pub fn problem(digits: usize) -> i64 {
    let max = (10 as i64).pow(digits as u32);
    let min = max / 10;
    let mut result = 0;

    for left in (min..max).rev() {
        for right in (left..max).rev() {
            let current = left * right;
            if current < result {
                break;
            } else if is_palindrome_number(current) {
                result = current;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_5_digits() {
        assert_eq!(problem(5), 9_966_006_699);
    }

    #[test]
    fn for_4_digits() {
        assert_eq!(problem(4), 99_000_099);
    }

    #[test]
    fn for_3_digits() {
        assert_eq!(problem(3), 906_609);
    }

    #[test]
    fn for_2_digits() {
        assert_eq!(problem(2), 9009);
    }

    #[test]
    fn is_not_palindrome_909209() {
        assert_eq!(is_palindrome_number(909_209), false);
    }

    #[test]
    fn is_palindrome_90909() {
        assert_eq!(is_palindrome_number(90909), true);
    }
}
