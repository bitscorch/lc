//! 67. Add Binary
//! Easy | Math | String | Bit Manipulation | Simulation
//! https://leetcode.com/problems/add-binary/
//!
//! Given two binary strings `a` and `b`, return *their sum as a binary string*.
//!
//! **Example 1:**
//!
//! ```
//! Input: a = "11", b = "1"
//! Output: "100"
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: a = "1010", b = "1011"
//! Output: "10101"
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= a.length, b.length <= 10<sup>4</sup>`
//! * `a` and `b` consist only of `'0'` or `'1'` characters.
//! * Each string does not contain leading zeros except for the zero itself.

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ai = a.bytes().rev();
        let mut bi = b.bytes().rev();

        let mut carry = 0u8;
        let mut ans = Vec::new();

        loop {
            match (ai.next(), bi.next()) {
                (None, None) if carry == 0 => break,
                (x, y) => {
                    let sum = x.map_or(0, |b| b - b'0') + y.map_or(0, |b| b - b'0') + carry;
                    ans.push(b'0' + sum % 2);
                    carry = sum / 2;
                }
            }
        }

        ans.reverse();
        String::from_utf8(ans).unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("11".to_string(), "1".to_string(), "100".to_string())]
    #[case("1010".to_string(), "1011".to_string(), "10101".to_string())]
    fn cases(#[case] a: String, #[case] b: String, #[case] expected: String) {
        assert_eq!(expected, Solution::add_binary(a, b));
    }
}
