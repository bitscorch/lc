//! 1137. N-th Tribonacci Number
//! Easy | Math | Dynamic Programming | Memoization
//! https://leetcode.com/problems/n-th-tribonacci-number/
//!
//! The Tribonacci sequence T<sub>n</sub> is defined as follows:
//!
//! T<sub>0</sub> = 0, T<sub>1</sub> = 1, T<sub>2</sub> = 1, and T<sub>n+3</sub>
//! = T<sub>n</sub> + T<sub>n+1</sub> + T<sub>n+2</sub> for n \>= 0.
//!
//! Given `n`, return the value of T<sub>n</sub>.
//!
//! **Example 1:**
//!
//! ```
//! Input: n = 4
//! Output: 4
//! Explanation:
//! T_3 = 0 + 1 + 1 = 2
//! T_4 = 1 + 1 + 2 = 4
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: n = 25
//! Output: 1389537
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= n <= 37`
//! * The answer is guaranteed to fit within a 32-bit integer, ie. `answer <= 2^31 - 1`.

struct Solution;

// t-1 = 0
// t0 = 0
// t1 = 1
// t2 = t-1 + t0 + t1 = 1
// t3 = t0 + t1 + t2 = 2
// t4 = t1 + t2 + t3 = 4
// t5 = t2 + t3 + t4 = 7
//
// T_3 = t_0 + 1 + 1 = 2
// T_4 = 1 + 1 + 2 = 4
// T_5 = 1 + 1 + 2 = 4
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 || n == 2 {
            return 1;
        };

        let (mut a, mut b, mut c) = (0, 1, 1);
        for _ in 2..n {
            (a, b, c) = (b, c, a + b + c);
        }
        c
    }
}

fn main() {
    Solution::tribonacci(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Inputs are filled from LeetCode's examples; replace each `0` with the
    // expected output (see the `Output:` lines in the description above).
    #[test]
    fn case_1() {
        assert_eq!(4, Solution::tribonacci(4));
    }

    #[test]
    fn case_2() {
        assert_eq!(1389537, Solution::tribonacci(25));
    }
}
