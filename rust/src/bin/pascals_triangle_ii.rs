//! 119. Pascal's Triangle II
//! Easy | Array | Dynamic Programming
//! https://leetcode.com/problems/pascals-triangle-ii/
//!
//! Given an integer `rowIndex`, return the `rowIndex<sup>th</sup>`
//! (**0-indexed**) row of the **Pascal's triangle**.
//!
//! In **Pascal's triangle**, each number is the sum of the two numbers directly
//! above it as shown:
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)
//!
//! **Example 1:**
//!
//! ```
//! Input: rowIndex = 3
//! Output: [1,3,3,1]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: rowIndex = 0
//! Output: [1]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: rowIndex = 1
//! Output: [1,1]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= rowIndex <= 33`
//!
//! **Follow up:** Could you optimize your algorithm to use only `O(rowIndex)` extra space?

struct Solution;

// n = 4
// [1, 1, 1, 1]
// [1]
// [1, 1]
// [1, 2 ,1]
// [1, 3, 3 ,1]
// [1, 4, 6, 4, 1]
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row: Vec<i32> = vec![1; 1 + row_index as usize];

        for i in 2..=row_index as usize {
            for j in (1..i).rev() {
                row[j] += row[j - 1];
            }
        }
        row
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3, vec![1, 3, 3, 1])]
    #[case(0, vec![1])]
    #[case(1, vec![1, 1])]
    fn cases(#[case] row_index: i32, #[case] expected: Vec<i32>) {
        assert_eq!(expected, Solution::get_row(row_index));
    }
}
