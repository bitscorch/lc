//! 70. Climbing Stairs
//! Easy | Math | Dynamic Programming | Memoization
//! https://leetcode.com/problems/climbing-stairs/
//!
//! You are climbing a staircase. It takes `n` steps to reach the top.
//!
//! Each time you can either climb `1` or `2` steps. In how many distinct ways
//! can you climb to the top?
//!
//! **Example 1:**
//!
//! ```
//! Input: n = 2
//! Output: 2
//! Explanation: There are two ways to climb to the top.
//! 1. 1 step + 1 step
//! 2. 2 steps
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: n = 3
//! Output: 3
//! Explanation: There are three ways to climb to the top.
//! 1. 1 step + 1 step + 1 step
//! 2. 1 step + 2 steps
//! 3. 2 steps + 1 step
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= n <= 45`

struct Solution;

// f(1) = [1] = 1
// f(2) = [1, 1], [2] = 2
// f(3) = [1, 1, 1], [2, 1], [1, 2] = 3
// f(4) = [1, 1, 1, 1], [2, 1, 1], [1, 1, 2], [1, 2, 1], [2, 2] = 5
//
// f(n) = f(n - 1) + f(n - 2) (fibonacci)
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut prev, mut curr) = (1, 1);

        for _ in 1..n {
            (prev, curr) = (curr, prev + curr);
        }

        curr
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Inputs are filled from LeetCode's examples; replace each `0` with the
    // expected output (see the `Output:` lines in the description above).
    #[test]
    fn case_1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn case_2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
}
