//! 3. Longest Substring Without Repeating Characters
//! Medium | Hash Table | String | Sliding Window
//! https://leetcode.com/problems/longest-substring-without-repeating-characters/
//!
//! Given a string `s`, find the length of the **longest** **substring** without
//! duplicate characters.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "abcabcbb"
//! Output: 3
//! Explanation: The answer is "abc", with the length of 3. Note that "bca" and "cab" are also correct answers.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "bbbbb"
//! Output: 1
//! Explanation: The answer is "b", with the length of 1.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: s = "pwwkew"
//! Output: 3
//! Explanation: The answer is "wke", with the length of 3.
//! Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= s.length <= 5 * 10<sup>4</sup>`
//! * `s` consists of English letters, digits, symbols and spaces.

struct Solution;

// 2 pointer O(N) with counts
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        };
        let bytes = s.as_bytes();
        let (mut p1, mut p2, mut l) = (0, 0, 1);
        let mut counts = [0; 256];
        counts[bytes[0] as usize] = 1;

        while p2 < bytes.len() - 1 {
            p2 += 1;
            counts[bytes[p2] as usize] += 1;
            // println!(
            //     "{} | {}",
            //     bytes[p2] as char,
            //     counts[(bytes[p2] - b'a') as usize]
            // );
            while counts[bytes[p2] as usize] > 1 {
                counts[bytes[p1] as usize] -= 1;
                p1 += 1;
            }
            // println!("{p1} | {p2} | {l}");
            l = l.max(p2 - p1 + 1)
        }

        l as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcabcbb".to_string(), 3)]
    #[case("bbbbb".to_string(), 1)]
    #[case("pwwkew".to_string(), 3)]
    #[case("".to_string(), 0)]
    #[case("[]  []".to_string(), 3)]
    fn cases(#[case] s: String, #[case] expected: i32) {
        assert_eq!(expected, Solution::length_of_longest_substring(s));
    }
}
