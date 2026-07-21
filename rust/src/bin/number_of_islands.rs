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

fn flood_fill(grid: &Vec<Vec<char>>, vis: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    if vis[i][j] {
        return;
    } else if grid[i][j] == '0' {
        vis[i][j] = true;
        return;
    }

    vis[i][j] = true;

    flood_fill(grid, vis, (i + 1).min(grid.len() - 1), j);
    flood_fill(grid, vis, i.saturating_sub(1), j);
    flood_fill(grid, vis, i, (j + 1).min(grid[0].len() - 1));
    flood_fill(grid, vis, i, j.saturating_sub(1));
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;
        let mut vis = vec![vec![false; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if (grid[i][j] == '1') & !vis[i][j] {
                    islands += 1;
                    flood_fill(&grid, &mut vis, i, j);
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
