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

use std::collections::VecDeque;

//   u
// l . r
//   d
// stack then reverse when zero hit?
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::with_capacity(mat.len() * mat[0].len());
        let mut ans = mat.clone();

        for (i, row) in ans.iter_mut().enumerate() {
            for (j, val) in row.iter_mut().enumerate() {
                if *val == 0 {
                    queue.push_back((i, j));
                } else {
                    *val = -1;
                }
            }
        }

        // 0, 1, 2 -> len 3
        // i < 3 - 1
        while let Some((i, j)) = queue.pop_front() {
            if i != 0 && ans[i - 1][j] == -1 {
                ans[i - 1][j] = ans[i][j] + 1;
                queue.push_back((i - 1, j));
            }
            if i != ans.len() - 1 && ans[i + 1][j] == -1 {
                ans[i + 1][j] = ans[i][j] + 1;
                queue.push_back((i + 1, j));
            }
            if j != 0 && ans[i][j - 1] == -1 {
                ans[i][j - 1] = ans[i][j] + 1;
                queue.push_back((i, j - 1));
            }
            if j != ans[i].len() - 1 && ans[i][j + 1] == -1 {
                ans[i][j + 1] = ans[i][j] + 1;
                queue.push_back((i, j + 1));
            }

            // println!("{i} | {j}");
            // println!("{ans:?}");
        }

        ans
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
