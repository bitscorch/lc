//! 338. Counting Bits
//! Easy | Dynamic Programming | Bit Manipulation
//! https://leetcode.com/problems/counting-bits/
//!
//! Given an integer `n`, return *an array* `ans` *of length* `n + 1` *such that
//! for each* `i` (`0 <= i <= n`)*,* `ans[i]` *is the **number of*** `1`***'s**
//! in the binary representation of* `i`.
//!
//! **Example 1:**
//!
//! ```
//! Input: n = 2
//! Output: [0,1,1]
//! Explanation:
//! 0 --> 0
//! 1 --> 1
//! 2 --> 10
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: n = 5
//! Output: [0,1,1,2,1,2]
//! Explanation:
//! 0 --> 0
//! 1 --> 1
//! 2 --> 10
//! 3 --> 11
//! 4 --> 100
//! 5 --> 101
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= n <= 10<sup>5</sup>`
//!
//! **Follow up:**
//!
//! * It is very easy to come up with a solution with a runtime of `O(n log n)`. Can you do it in linear time `O(n)` and possibly in a single pass?
//! * Can you do it without using any built-in function (i.e., like `__builtin_popcount` in C++)?

struct Solution;

// 0  => 0    -> 0
// 1  => 1    -> 1
// 2  => 10   -> 1
// 3  => 11   -> 2
// 4  => 100  -> 1
// 5  => 101  -> 2
// 6  => 110  -> 2
// 7  => 111  -> 3
// 8  => 1000 -> 1
// 9  => 1001 -> 2
// 10 => 1010 -> 2
// 11 => 1011 -> 3
// 12 => 1100 -> 2
// 13 => 1101 -> 3
// 14 => 1111 -> 4
//
//  bits[i] = i / 2 + i % 2
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut bits = Vec::with_capacity((n + 1) as usize);
        bits.push(0);

        for i in 1..n + 1 {
            bits.push(bits[(i >> 1) as usize] + (i & 1));
        }

        bits
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
        assert_eq!(vec![0, 1, 1], Solution::count_bits(2));
    }

    #[test]
    fn case_2() {
        assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::count_bits(5));
    }
}
