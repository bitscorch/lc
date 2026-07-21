//! 994. Rotting Oranges
//! Medium | Array | Breadth-First Search | Matrix
//! https://leetcode.com/problems/rotting-oranges/
//!
//! You are given an `m x n` `grid` where each cell can have one of three
//! values:
//!
//! * `0` representing an empty cell,
//! * `1` representing a fresh orange, or
//! * `2` representing a rotten orange.
//!
//! Every minute, any fresh orange that is **4-directionally adjacent** to a
//! rotten orange becomes rotten.
//!
//! Return *the minimum number of minutes that must elapse until no cell has a
//! fresh orange*. If *this is impossible, return* `-1`.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2019/02/16/oranges.png)
//!
//! ```
//! Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
//! Output: 4
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
//! Output: -1
//! Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: grid = [[0,2]]
//! Output: 0
//! Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == grid.length`
//! * `n == grid[i].length`
//! * `1 <= m, n <= 10`
//! * `grid[i][j]` is `0`, `1`, or `2`.

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut queue = VecDeque::with_capacity(rows * cols);
        let mut ans = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == 2 {
                    queue.push_back((i, j, 0));
                }
            }
        }

        while let Some((i, j, turn)) = queue.pop_front() {
            ans = ans.max(turn);

            for (di, dj) in [(-1isize, 0), (1, 0), (0, -1), (0, 1)] {
                let (Some(ni), Some(nj)) = (i.checked_add_signed(di), j.checked_add_signed(dj))
                else {
                    continue;
                };

                if ni < rows && nj < cols && grid[ni][nj] == 1 {
                    grid[ni][nj] = 2;
                    queue.push_back((ni, nj, turn + 1));
                }
            }
        }

        if grid.iter().flatten().any(|&c| c == 1) {
            -1
        } else {
            ans
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![2,1,1],vec![1,1,0],vec![0,1,1]], 4)]
    #[case(vec![vec![2,1,1],vec![0,1,1],vec![1,0,1]], -1)]
    #[case(vec![vec![0,2]], 0)]
    fn cases(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        assert_eq!(expected, Solution::oranges_rotting(grid));
    }
}
