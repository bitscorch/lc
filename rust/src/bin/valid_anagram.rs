//! 242. Valid Anagram
//! Easy | Hash Table | String | Sorting
//! https://leetcode.com/problems/valid-anagram/
//!
//! Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`,
//! and `false` otherwise.
//!
//! **Example 1:**
//!
//! **Input:** s = "anagram", t = "nagaram"
//!
//! **Output:** true
//!
//! **Example 2:**
//!
//! **Input:** s = "rat", t = "car"
//!
//! **Output:** false
//!
//! **Constraints:**
//!
//! * `1 <= s.length, t.length <= 5 * 10<sup>4</sup>`
//! * `s` and `t` consist of lowercase English letters.
//!
//! **Follow up:** What if the inputs contain Unicode characters? How would you
//! adapt your solution to such a case?

struct Solution;

// TODO: check
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // For follow up, change bytes to chars and use a HashMap instead of arr
        let (s, t) = (s.as_bytes(), t.as_bytes());
        if s.len() != t.len() {
            return false;
        }

        let mut d = [0; (b'z' - b'a' + 1) as usize];
        for i in 0..s.len() {
            d[(s[i] - b'a') as usize] += 1;
            d[(t[i] - b'a') as usize] -= 1;
        }

        d.iter().all(|x| *x == 0)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("anagram".to_string(), "nagaram".to_string(), true)]
    #[case("rat".to_string(), "car".to_string(), false)]
    fn cases(#[case] s: String, #[case] t: String, #[case] expected: bool) {
        assert_eq!(expected, Solution::is_anagram(s, t));
    }
}
