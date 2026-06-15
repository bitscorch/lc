//! 20. Valid Parentheses
//! Easy | String | Stack
//! https://leetcode.com/problems/valid-parentheses/
//!
//! Given a string `s` containing just the characters `'('`, `')'`, `'{'`,
//! `'}'`, `'['` and `']'`, determine if the input string is valid.
//!
//! An input string is valid if:
//!
//! 1. Open brackets must be closed by the same type of brackets.
//! 2. Open brackets must be closed in the correct order.
//! 3. Every close bracket has a corresponding open bracket of the same type.
//!
//! **Example 1:**
//!
//! **Input:** s = "()"
//!
//! **Output:** true
//!
//! **Example 2:**
//!
//! **Input:** s = "()[]{}"
//!
//! **Output:** true
//!
//! **Example 3:**
//!
//! **Input:** s = "(]"
//!
//! **Output:** false
//!
//! **Example 4:**
//!
//! **Input:** s = "([])"
//!
//! **Output:** true
//!
//! **Example 5:**
//!
//! **Input:** s = "([)]"
//!
//! **Output:** false
//!
//! **Constraints:**
//!
//! * `1 <= s.length <= 10<sup>4</sup>`
//! * `s` consists of parentheses only `'()[]{}'`.

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.bytes() {
            match c {
                b'(' => stack.push(b')'),
                b'{' => stack.push(b'}'),
                b'[' => stack.push(b']'),
                _ if stack.pop() != Some(c) => return false,
                _ => {}
            }
        }
        stack.is_empty()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("()".to_string(), true)]
    #[case("()[]{}".to_string(), true)]
    #[case("(]".to_string(), false)]
    #[case("([])".to_string(), true)]
    #[case("([)]".to_string(), false)]
    #[case("((".to_string(), false)]
    fn cases(#[case] s: String, #[case] expected: bool) {
        assert_eq!(expected, Solution::is_valid(s));
    }
}
