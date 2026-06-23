//! 383. Ransom Note
//! Easy | Hash Table | String | Counting
//! https://leetcode.com/problems/ransom-note/
//!
//! Given two strings `ransomNote` and `magazine`, return `true` *if*
//! `ransomNote` *can be constructed by using the letters from* `magazine` *and*
//! `false` *otherwise*.
//!
//! Each letter in `magazine` can only be used once in `ransomNote`.
//!
//! **Example 1:**
//!
//! ```
//! Input: ransomNote = "a", magazine = "b"
//! Output: false
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: ransomNote = "aa", magazine = "ab"
//! Output: false
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: ransomNote = "aa", magazine = "aab"
//! Output: true
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= ransomNote.length, magazine.length <= 10<sup>5</sup>`
//! * `ransomNote` and `magazine` consist of lowercase English letters.

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut counts = [0i32; 26];
        for c in magazine.bytes() {
            counts[(c - b'a') as usize] += 1
        }
        ransom_note.bytes().all(|c| {
            let i = (c - b'a') as usize;
            counts[i] -= 1;
            counts[i] >= 0
        })
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("a".to_string(), "b".to_string(), false)]
    #[case("aa".to_string(), "ab".to_string(), false)]
    #[case("aa".to_string(), "aab".to_string(), true)]
    fn cases(#[case] ransom_note: String, #[case] magazine: String, #[case] expected: bool) {
        assert_eq!(expected, Solution::can_construct(ransom_note, magazine));
    }
}
