//! 1189. Maximum Number of Balloons
//! Easy | Hash Table | String | Counting
//! https://leetcode.com/problems/maximum-number-of-balloons/
//!
//! Given a string `text`, you want to use the characters of `text` to form as
//! many instances of the word **"balloon"** as possible.
//!
//! You can use each character in `text` **at most once**. Return the maximum
//! number of instances that can be formed.
//!
//! **Example 1:**
//!
//! **![](https://assets.leetcode.com/uploads/2019/09/05/1536_ex1_upd.JPG)**
//!
//! ```
//! Input: text = "nlaebolko"
//! Output: 1
//!
//! ```
//!
//! **Example 2:**
//!
//! **![](https://assets.leetcode.com/uploads/2019/09/05/1536_ex2_upd.JPG)**
//!
//! ```
//! Input: text = "loonbalxballpoon"
//! Output: 2
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: text = "leetcode"
//! Output: 0
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= text.length <= 10<sup>4</sup>`
//! * `text` consists of lower case English letters only.
//!
//! **Note:** This question is the same as [ 2287: Rearrange Characters to Make
//! Target
//! String.](https://leetcode.com/problems/rearrange-characters-to-make-target-string/description/)

struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut cnts = [0; 5];

        for c in text.bytes() {
            match c {
                b'b' => cnts[0] += 1,
                b'a' => cnts[1] += 1,
                b'l' => cnts[2] += 1,
                b'o' => cnts[3] += 1,
                b'n' => cnts[4] += 1,
                _ => {}
            };
        }
        cnts[2] /= 2;
        cnts[3] /= 2;

        *cnts.iter().min().unwrap_or(&0)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("nlaebolko".to_string(), 1)]
    #[case("loonbalxballpoon".to_string(), 2)]
    #[case("leetcode".to_string(), 0)]
    fn cases(#[case] text: String, #[case] expected: i32) {
        assert_eq!(expected, Solution::max_number_of_balloons(text));
    }
}
