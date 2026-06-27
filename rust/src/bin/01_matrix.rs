//! 542. 01 Matrix
//! Medium | Array | Dynamic Programming | Breadth-First Search | Matrix
//! https://leetcode.com/problems/01-matrix/
//!
//! Given an `m x n` binary matrix `mat`, return *the distance of the nearest*
//! `0` *for each cell*.
//!
//! The distance between two cells sharing a common edge is `1`.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/04/24/01-1-grid.jpg)
//!
//! ```
//! Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
//! Output: [[0,0,0],[0,1,0],[0,0,0]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/04/24/01-2-grid.jpg)
//!
//! ```
//! Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
//! Output: [[0,0,0],[0,1,0],[1,2,1]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == mat.length`
//! * `n == mat[i].length`
//! * `1 <= m, n <= 10<sup>4</sup>`
//! * `1 <= m * n <= 10<sup>4</sup>`
//! * `mat[i][j]` is either `0` or `1`.
//! * There is at least one `0` in `mat`.
//!
//! **Note:** This question is the same as 1765:
//! [https://leetcode.com/problems/map-of-highest-peak/](https://leetcode.com/problems/map-of-highest-peak/description/)

struct Solution;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (mat.len(), mat[0].len());
        const MAX: i32 = i32::MAX - 1;

        for row in mat.iter_mut() {
            for val in row.iter_mut() {
                if *val == 1 {
                    *val = MAX
                };
            }
        }

        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] == 0 {
                    continue;
                };
                let up = if i > 0 { mat[i - 1][j] } else { MAX };
                let left = if j > 0 { mat[i][j - 1] } else { MAX };
                mat[i][j] = mat[i][j].min(up.min(left) + 1);
            }
        }

        for i in (0..rows).rev() {
            for j in (0..cols).rev() {
                if mat[i][j] == 0 {
                    continue;
                }
                let down = if i + 1 < rows { mat[i + 1][j] } else { MAX };
                let right = if j + 1 < cols { mat[i][j + 1] } else { MAX };
                mat[i][j] = mat[i][j].min(down.min(right) + 1)
            }
        }

        mat
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]], vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]])]
    #[case(vec![vec![0,0,0],vec![0,1,0],vec![1,1,1]], vec![vec![0,0,0],vec![0,1,0],vec![1,2,1]])]
    #[case(vec![vec![1,1,1],vec![1,1,1],vec![0,0,0]], vec![vec![2,2,2],vec![1,1,1],vec![0,0,0]])]
    #[case(vec![
        vec![0,1,0,1,1],
        vec![1,1,0,0,1],
        vec![0,0,0,1,0],
        vec![1,0,1,1,1],
        vec![1,0,0,0,1]
    ], vec![
        vec![0,1,0,1,2],
        vec![1,1,0,0,1],
        vec![0,0,0,1,0],
        vec![1,0,1,1,1],
        vec![1,0,0,0,1]
    ])]
    fn cases(#[case] mat: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        assert_eq!(expected, Solution::update_matrix(mat));
    }
}
