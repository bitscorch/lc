//! 509. Fibonacci Number
//! Easy | Math | Dynamic Programming | Recursion | Memoization
//! https://leetcode.com/problems/fibonacci-number/
//!
//! The **Fibonacci numbers**, commonly denoted `F(n)` form a sequence, called
//! the **Fibonacci sequence**, such that each number is the sum of the two
//! preceding ones, starting from `0` and `1`. That is,
//!
//! ```
//! F(0) = 0, F(1) = 1
//! F(n) = F(n - 1) + F(n - 2), for n > 1.
//!
//! ```
//!
//! Given `n`, calculate `F(n)`.
//!
//! **Example 1:**
//!
//! ```
//! Input: n = 2
//! Output: 1
//! Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: n = 3
//! Output: 2
//! Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: n = 4
//! Output: 3
//! Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= n <= 30`

struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let (mut a, mut b) = (0, 1);
        for _ in 0..n {
            (a, b) = (b, a + b);
        }
        a
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(4, 3)]
    fn cases(#[case] n: i32, #[case] expected: i32) {
        assert_eq!(expected, Solution::fib(n));
    }
}
