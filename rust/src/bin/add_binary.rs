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
        let n = a.len().max(b.len());
        let mut ai = a.bytes().rev();
        let mut bi = b.bytes().rev();

        let mut carry = b'0';

        let mut ans: Vec<u8> = (0..n)
            .map(|_| {
                let x = ai.next().unwrap_or(b'0') - b'0';
                let y = bi.next().unwrap_or(b'0') - b'0';

                let sum = x + y + carry - b'0';
                if sum == 3 {
                    carry = b'1';
                    b'1'
                } else if sum == 2 {
                    carry = b'1';
                    b'0'
                } else if sum == 1 {
                    carry = b'0';
                    b'1'
                } else {
                    carry = b'0';
                    b'0'
                }
            })
            .collect();

        if carry == b'1' {
            ans.push(b'1')
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
