//! 392. Is Subsequence
//! Easy | Two Pointers | String | Dynamic Programming
//! https://leetcode.com/problems/is-subsequence/
//!
//! Given two strings `s` and `t`, return `true` *if* `s` *is a **subsequence**
//! of* `t`*, or* `false` *otherwise*.
//!
//! A **subsequence** of a string is a new string that is formed from the
//! original string by deleting some (can be none) of the characters without
//! disturbing the relative positions of the remaining characters. (i.e.,
//! `"ace"` is a subsequence of `"abcde"` while `"aec"` is not).
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "abc", t = "ahbgdc"
//! Output: true
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "axc", t = "ahbgdc"
//! Output: false
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= s.length <= 100`
//! * `0 <= t.length <= 10<sup>4</sup>`
//! * `s` and `t` consist only of lowercase English letters.
//!
//! **Follow up:** Suppose there are lots of incoming `s`, say `s<sub>1</sub>, s<sub>2</sub>, ..., s<sub>k</sub>` where `k >= 10<sup>9</sup>`, and you want to check one by one to see if `t` has its subsequence. In this scenario, how would you change your code?

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut it = t.bytes();
        s.bytes().all(|c| it.any(|d| d == c))
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abc".to_string(), "ahbgdc".to_string(), true)]
    #[case("axc".to_string(), "ahbgdc".to_string(), false)]
    fn cases(#[case] s: String, #[case] t: String, #[case] expected: bool) {
        assert_eq!(expected, Solution::is_subsequence(s, t));
    }
}
