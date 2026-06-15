//! 125. Valid Palindrome
//! Easy | Two Pointers | String
//! https://leetcode.com/problems/valid-palindrome/
//!
//! A phrase is a **palindrome** if, after converting all uppercase letters into
//! lowercase letters and removing all non-alphanumeric characters, it reads the
//! same forward and backward. Alphanumeric characters include letters and
//! numbers.
//!
//! Given a string `s`, return `true` *if it is a **palindrome**, or* `false`
//! *otherwise*.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "A man, a plan, a canal: Panama"
//! Output: true
//! Explanation: "amanaplanacanalpanama" is a palindrome.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "race a car"
//! Output: false
//! Explanation: "raceacar" is not a palindrome.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: s = " "
//! Output: true
//! Explanation: s is an empty string "" after removing non-alphanumeric characters.
//! Since an empty string reads the same forward and backward, it is a palindrome.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= s.length <= 2 * 10<sup>5</sup>`
//! * `s` consists only of printable ASCII characters.

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let (mut i, mut j) = (0, s.len() - 1);

        while i < j {
            if !s[i].is_ascii_alphanumeric() {
                i += 1
            } else if !s[j].is_ascii_alphanumeric() {
                j -= 1
            } else if !s[i].eq_ignore_ascii_case(&s[j]) {
                return false;
            } else {
                i += 1;
                j -= 1;
            }
        }
        true
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("A man, a plan, a canal: Panama".to_string(), true)]
    #[case("race a car".to_string(), false)]
    #[case(" ".to_string(), true)]
    #[case("0P".to_string(), false)]
    fn cases(#[case] s: String, #[case] expected: bool) {
        assert_eq!(expected, Solution::is_palindrome(s));
    }
}
