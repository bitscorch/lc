//! 1358. Number of Substrings Containing All Three Characters
//! Medium | Hash Table | String | Sliding Window
//! https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
//!
//! Given a string `s` consisting only of characters *a*, *b* and *c*.
//!
//! Return the number of substrings containing **at least** one occurrence of
//! all these characters *a*, *b* and *c*.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "abcabc"
//! Output: 10
//! Explanation: The substrings containing at least one occurrence of the characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again).
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "aaacb"
//! Output: 3
//! Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb".
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: s = "abc"
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `3 <= s.length <= 5 x 10^4`
//! * `s` only consists of *a*, *b* or *c* characters.

struct Solution;

// abcde 5
// abc tttttttttttttttttttttttttttttttt
// aabbc
//
// abcabc
// 0, 2 -> abc -> 4
// 1, 3 -> bca -> 4
// 2, 4 -> cab -> 4
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let b = s.as_bytes();
        let mut ans = 0;
        let mut seen = [-1; 3];

        for i in 0..b.len() {
            seen[(b[i] - b'a') as usize] = i as i32;

            if seen[0] != -1 && seen[1] != -1 && seen[2] != -1 {
                ans += 1 + seen[0].min(seen[1]).min(seen[2]);
            }
        }

        ans
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcabc".to_string(), 10)]
    #[case("aaacb".to_string(), 3)]
    #[case("abc".to_string(), 1)]
    fn cases(#[case] s: String, #[case] expected: i32) {
        assert_eq!(expected, Solution::number_of_substrings(s));
    }
}
