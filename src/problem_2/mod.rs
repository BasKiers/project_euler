#![allow(dead_code)]

/**
 * Each new term in the Fibonacci sequence is generated by adding the previous two terms.
 * By starting with 1 and 2, the first 10 terms will be:
 * 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
 * By considering the terms in the Fibonacci sequence whose values do not exceed four million,
 * find the sum of the even-valued terms.
 */

struct Fib {
    last_values: (usize, usize),
}

impl Fib {
    fn new() -> Fib {
        Fib {
            last_values: (0, 1),
        }
    }
}

impl Iterator for Fib {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.last_values = (self.last_values.1, self.last_values.0 + self.last_values.1);
        Some(self.last_values.0)
    }
}

pub fn problem(size: usize) -> usize {
    Fib::new()
        .take_while(|&val| val <= size)
        .fold(0, |acc, val| if val % 2 == 0 { acc + val } else { acc })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_1_000_000() {
        assert_eq!(problem(1_000_000), 1_089_154);
    }

    #[test]
    fn for_8() {
        assert_eq!(problem(8), 10);
    }

    #[test]
    fn for_0() {
        assert_eq!(problem(0), 0);
    }
}
