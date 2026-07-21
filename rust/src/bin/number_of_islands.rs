//! 200. Number of Islands
//! Medium | Array | Depth-First Search | Breadth-First Search | Union-Find | Matrix
//! https://leetcode.com/problems/number-of-islands/
//!
//! Given an `m x n` 2D binary grid `grid` which represents a map of `'1'`s
//! (land) and `'0'`s (water), return *the number of islands*.
//!
//! An **island** is surrounded by water and is formed by connecting adjacent
//! lands horizontally or vertically. You may assume all four edges of the grid
//! are all surrounded by water.
//!
//! **Example 1:**
//!
//! ```
//! Input: grid = [
//!   ["1","1","1","1","0"],
//!   ["1","1","0","1","0"],
//!   ["1","1","0","0","0"],
//!   ["0","0","0","0","0"]
//! ]
//! Output: 1
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: grid = [
//!   ["1","1","0","0","0"],
//!   ["1","1","0","0","0"],
//!   ["0","0","1","0","0"],
//!   ["0","0","0","1","1"]
//! ]
//! Output: 3
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == grid.length`
//! * `n == grid[i].length`
//! * `1 <= m, n <= 300`
//! * `grid[i][j]` is `'0'` or `'1'`.

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut vis = vec![vec![false; cols]; rows];
        let mut islands = 0;

        let mut stack = Vec::with_capacity(rows * cols);

        for i in 0..rows {
            for j in 0..cols {
                if (grid[i][j] == '0') || vis[i][j] {
                    continue;
                }

                islands += 1;
                // flood fill on the heap
                vis[i][j] = true;
                stack.push((i, j));

                while let Some((i, j)) = stack.pop() {
                    for (di, dj) in [(-1isize, 0), (1, 0), (0, -1), (0, 1)] {
                        let (Some(ni), Some(nj)) =
                            (i.checked_add_signed(di), j.checked_add_signed(dj))
                        else {
                            continue;
                        };

                        if ni < rows && nj < cols && grid[ni][nj] == '1' && !vis[ni][nj] {
                            vis[ni][nj] = true;
                            stack.push((ni, nj));
                        }
                    }
                }
            }
        }

        islands
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec!['1','1','1','1','0'],vec!['1','1','0','1','0'],vec!['1','1','0','0','0'],vec!['0','0','0','0','0']], 1)]
    #[case(vec![vec!['1','1','0','0','0'],vec!['1','1','0','0','0'],vec!['0','0','1','0','0'],vec!['0','0','0','1','1']], 3)]
    fn cases(#[case] grid: Vec<Vec<char>>, #[case] expected: i32) {
        assert_eq!(expected, Solution::num_islands(grid));
    }
}
