//! 118. Pascal's Triangle
//! Easy | Array | Dynamic Programming
//! https://leetcode.com/problems/pascals-triangle/
//!
//! Given an integer `numRows`, return the first numRows of **Pascal's
//! triangle**.
//!
//! In **Pascal's triangle**, each number is the sum of the two numbers directly
//! above it as shown:
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif)
//!
//! **Example 1:**
//!
//! ```
//! Input: numRows = 5
//! Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: numRows = 1
//! Output: [[1]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= numRows <= 30`

struct Solution;

// stop conditions:
// i == 1 => [1] and i == 2 => [1, 1]
// if j == 0 tri[i][j] = 1 Ok, so we kind of don't need the top two
//
// tri[i][j] = tri[i-1][j] + tri[i-1][j+1]
//
// if j == 0 { tri[i][j] = 1} else if j == i { tri[i][j] = 1 }
// else { tri[i][j] = tri[i-1][j] + tri[i - 1][j + 1] }
//
// 1 = [1]
// 2 = [1, 1]
// 3 = [1, 2, 1]
// 4 = [1, 3, 3, 1]
// 5 = [1, 4, 6, 4, 1]
// 6 = [1, 5, 10, 10, 5, 1]
//
// 0..1
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut tri: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
        for i in 0..num_rows {
            let mut curr = Vec::with_capacity((i + 1) as usize);
            curr.push(1);
            if let Some(prev) = tri.last() {
                curr.extend(prev.windows(2).map(|w| w[0] + w[1]));
                curr.push(1);
            }
            tri.push(curr);
        }
        tri
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]])]
    #[case(1, vec![vec![1]])]
    fn cases(#[case] num_rows: i32, #[case] expected: Vec<Vec<i32>>) {
        assert_eq!(expected, Solution::generate(num_rows));
    }
}
