//! 78. Subsets
//! Medium | Array | Backtracking | Bit Manipulation
//! https://leetcode.com/problems/subsets/
//!
//! Given an integer array `nums` of **unique** elements, return *all possible*
//! *subsets* *(the power set)*.
//!
//! The solution set **must not** contain duplicate subsets. Return the solution
//! in **any order**.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,2,3]
//! Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [0]
//! Output: [[],[0]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10`
//! * `-10 <= nums[i] <= 10`
//! * All the numbers of `nums` are **unique**.

struct Solution;

fn backtrack(nums: &[i32], cur: &mut Vec<i32>, pos: usize, res: &mut Vec<Vec<i32>>) {
    res.push(cur.clone());
    for i in pos..nums.len() {
        cur.push(nums[i]);
        backtrack(nums, cur, i + 1, res);
        cur.pop();
    }
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut cur, mut res) = (vec![], vec![]);
        backtrack(&nums, &mut cur, 0, &mut res);
        res
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], vec![vec![],vec![1],vec![2],vec![1,2],vec![3],vec![1,3],vec![2,3],vec![1,2,3]])]
    #[case(vec![0], vec![vec![],vec![0]])]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: Vec<Vec<i32>>) {
        let canon = |mut v: Vec<Vec<i32>>| {
            v.iter_mut().for_each(|s| s.sort());
            v.sort();
            v
        };
        assert_eq!(canon(expected), canon(Solution::subsets(nums)));
    }
}
