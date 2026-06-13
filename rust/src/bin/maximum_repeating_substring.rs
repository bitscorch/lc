//! 1668. Maximum Repeating Substring
//! Easy | String | Dynamic Programming | String Matching
//! https://leetcode.com/problems/maximum-repeating-substring/
//!
//! For a string `sequence`, a string `word` is **`k`-repeating** if `word`
//! concatenated `k` times is a substring of `sequence`. The `word`'s **maximum
//! `k`-repeating value** is the highest value `k` where `word` is `k`-repeating
//! in `sequence`. If `word` is not a substring of `sequence`, `word`'s maximum
//! `k`-repeating value is `0`.
//!
//! Given strings `sequence` and `word`, return *the **maximum `k`-repeating
//! value** of `word` in `sequence`*.
//!
//! **Example 1:**
//!
//! ```
//! Input: sequence = "ababc", word = "ab"
//! Output: 2
//! Explanation: "abab" is a substring in "ababc".
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: sequence = "ababc", word = "ba"
//! Output: 1
//! Explanation: "ba" is a substring in "ababc". "baba" is not a substring in "ababc".
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: sequence = "ababc", word = "ac"
//! Output: 0
//! Explanation: "ac" is not a substring in "ababc".
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= sequence.length <= 100`
//! * `1 <= word.length <= 100`
//! * `sequence` and `word` contains only lowercase English letters.

struct Solution;

// ababc
// ab
// 01020
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let (sl, wl) = (sequence.len(), word.len());
        let mut max = 0;
        let mut dp = vec![0; sl + 1];

        for i in wl..=sl {
            dp[i] = if sequence[i - wl..i] == word {
                dp[i - wl] + 1
            } else {
                0
            };
            max = max.max(dp[i]);
        }
        max
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ababc".to_string(), "ab".to_string(), 2)]
    #[case("ababc".to_string(), "ba".to_string(), 1)]
    #[case("ababc".to_string(), "ac".to_string(), 0)]
    fn cases(#[case] sequence: String, #[case] word: String, #[case] expected: i32) {
        assert_eq!(expected, Solution::max_repeating(sequence, word));
    }
}
