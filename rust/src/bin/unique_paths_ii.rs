//! 63. Unique Paths II
//! Medium | Array | Dynamic Programming | Matrix
//! https://leetcode.com/problems/unique-paths-ii/
//!
//! You are given an `m x n` integer array `grid`. There is a robot initially
//! located at the **top-left corner** (i.e., `grid[0][0]`). The robot tries to
//! move to the **bottom-right corner** (i.e., `grid[m - 1][n - 1]`). The robot
//! can only move either down or right at any point in time.
//!
//! An obstacle and space are marked as `1` or `0` respectively in `grid`. A
//! path that the robot takes cannot include **any** square that is an obstacle.
//!
//! Return *the number of possible unique paths that the robot can take to reach
//! the bottom-right corner*.
//!
//! The testcases are generated so that the answer will be less than or equal to
//! `2 * 10<sup>9</sup>`.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/11/04/robot1.jpg)
//!
//! ```
//! Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
//! Output: 2
//! Explanation: There is one obstacle in the middle of the 3x3 grid above.
//! There are two ways to reach the bottom-right corner:
//! 1. Right -> Right -> Down -> Down
//! 2. Down -> Down -> Right -> Right
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/11/04/robot2.jpg)
//!
//! ```
//! Input: obstacleGrid = [[0,1],[0,0]]
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == obstacleGrid.length`
//! * `n == obstacleGrid[i].length`
//! * `1 <= m, n <= 100`
//! * `obstacleGrid[i][j]` is `0` or `1`.

struct Solution;

// [0, 0, 0]
// [0, 1, 0]
// [0, 0, 0]
//
// [1, 0, 0]
// [0, 0, 0]
// [0, 0, 0]
//
// [1, 1, 1]
// [1, 0, 1]
// [1, 1, 2]
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![0; n];
        dp[0] = 1;

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    dp[j] = 0;
                } else if j > 0 {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[n - 1]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Inputs are filled from LeetCode's examples; replace each `0` with the
    // expected output (see the `Output:` lines in the description above).
    #[test]
    fn case_1() {
        assert_eq!(
            2,
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            1,
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]])
        );
    }

    #[test]
    fn obstacle_on_top_row() {
        assert_eq!(
            0,
            Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0]])
        );
    }

    #[test]
    fn start_blocked() {
        assert_eq!(0, Solution::unique_paths_with_obstacles(vec![vec![1]]));
    }
}
