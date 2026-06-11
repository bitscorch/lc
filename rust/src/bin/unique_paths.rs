//! 62. Unique Paths
//! Medium | Math | Dynamic Programming | Combinatorics
//! https://leetcode.com/problems/unique-paths/
//!
//! There is a robot on an `m x n` grid. The robot is initially located at the
//! **top-left corner** (i.e., `grid[0][0]`). The robot tries to move to the
//! **bottom-right corner** (i.e., `grid[m - 1][n - 1]`). The robot can only
//! move either down or right at any point in time.
//!
//! Given the two integers `m` and `n`, return *the number of possible unique
//! paths that the robot can take to reach the bottom-right corner*.
//!
//! The test cases are generated so that the answer will be less than or equal
//! to `2 * 10<sup>9</sup>`.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png)
//!
//! ```
//! Input: m = 3, n = 7
//! Output: 28
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: m = 3, n = 2
//! Output: 3
//! Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
//! 1. Right -> Down -> Down
//! 2. Down -> Down -> Right
//! 3. Down -> Right -> Down
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= m, n <= 100`

struct Solution;

// 1 0 0
// 0 0 0
//
// R | D
// 1 1 0
// 1 0 0
//
// mov(&mut grid, pos: (i32, i32), mov: R | D) {
// if R {
// new_pos = (p.0 + 1, p.1);
// grid[new_pos.0][new_pos.1] = 1 + grid[pos.0][pos.1]
// mov(grid, new_pos, R);
// mov(grid, new_pos, D);
// } else { ... }
// }
//
// This is a bit too stack heavy
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize]; m as usize];

        for i in 0..m as usize {
            for j in 0..n as usize {
                dp[i][j] = match (i, j) {
                    (0, _) => 1,
                    (_, 0) => 1,
                    _ => dp[i - 1][j] + dp[i][j - 1],
                }
            }
        }

        dp[(m - 1) as usize][(n - 1) as usize]
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
        assert_eq!(28, Solution::unique_paths(3, 7));
    }

    #[test]
    fn case_2() {
        assert_eq!(3, Solution::unique_paths(3, 2));
    }
}
