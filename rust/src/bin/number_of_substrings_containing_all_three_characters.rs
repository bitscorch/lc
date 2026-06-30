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
        let n = b.len();
        let mut ans = 0;
        let mut counts = [0; 3];
        let (mut p1, mut p2) = (0, 0);

        loop {
            if counts.iter().all(|&x| x > 0) {
                ans += n - p2 + 1;
                counts[(b[p1] - b'a') as usize] -= 1;
                p1 += 1;
            } else if p2 < n {
                counts[(b[p2] - b'a') as usize] += 1;
                p2 += 1;
            } else {
                break;
            }
        }

        ans as i32
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
