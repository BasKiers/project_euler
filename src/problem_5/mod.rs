#![allow(dead_code)]

/**
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 */
pub fn problem(to_value: usize) -> usize {
    let mut highest_num = to_value;

    while (1..to_value).any(|val| highest_num % val != 0) {
        highest_num += to_value;
    }

    highest_num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_20() {
        assert_eq!(problem(20), 232_792_560);
    }

    #[test]
    fn for_10() {
        assert_eq!(problem(10), 2520);
    }
}
