//! 409. Longest Palindrome
//! Easy | Hash Table | String | Greedy
//! https://leetcode.com/problems/longest-palindrome/
//!
//! Given a string `s` which consists of lowercase or uppercase letters, return
//! the length of the **longest palindrome** that can be built with those
//! letters.
//!
//! Letters are **case sensitive**, for example, `"Aa"` is not considered a
//! palindrome.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "abccccdd"
//! Output: 7
//! Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "a"
//! Output: 1
//! Explanation: The longest palindrome that can be built is "a", whose length is 1.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= s.length <= 2000`
//! * `s` consists of lowercase **and/or** uppercase English letters only.

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnts = [0; 26 * 2];
        for c in s.bytes() {
            if c.is_ascii_lowercase() {
                cnts[(c - b'a') as usize] += 1;
            } else if c.is_ascii_uppercase() {
                cnts[(c - b'A' + 26) as usize] += 1;
            }
        }

        let (pair, odd) = cnts
            .iter()
            .fold((0, 0), |(pair, odd), &x| (pair + x / 2, odd.max(x % 2)));

        pair * 2 + odd
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abccccdd".to_string(), 7)]
    #[case("a".to_string(), 1)]
    fn cases(#[case] s: String, #[case] expected: i32) {
        assert_eq!(expected, Solution::longest_palindrome(s));
    }
}
