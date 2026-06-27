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
        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (rows, cols) = (mat.len() as i32, mat[0].len() as i32);
        let mut ans = mat;
        let mut queue = VecDeque::with_capacity((rows * cols) as usize);

        for i in 0..ans.len() {
            for j in 0..ans[0].len() {
                if ans[i][j] == 0 {
                    queue.push_back((i, j));
                } else {
                    ans[i][j] = -1
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            for (di, dj) in DIRS {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if ni < 0 || ni >= rows || nj < 0 || nj >= cols {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                if ans[ni][nj] == -1 {
                    ans[ni][nj] = ans[i][j] + 1;
                    queue.push_back((ni, nj));
                }
            }
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
