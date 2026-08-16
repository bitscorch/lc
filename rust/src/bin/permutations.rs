//! 46. Permutations
//! Medium | Array | Backtracking
//! https://leetcode.com/problems/permutations/
//!
//! Given an array `nums` of distinct integers, return all the possible
//! permutations. You can return the answer in **any order**.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,2,3]
//! Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [0,1]
//! Output: [[0,1],[1,0]]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [1]
//! Output: [[1]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 6`
//! * `-10 <= nums[i] <= 10`
//! * All the integers of `nums` are **unique**.

struct Solution;

fn backtrack(nums: &mut Vec<i32>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if nums.is_empty() {
        res.push(cur.clone());
        return;
    }

    for i in 0..nums.len() {
        cur.push(nums.remove(i));
        backtrack(nums, cur, res);
        nums.insert(i, cur.pop().unwrap());
    }
}

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut cur, mut res) = (vec![], vec![]);
        backtrack(&mut nums, &mut cur, &mut res);
        res
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,1,2],vec![3,2,1]])]
    #[case(vec![0,1], vec![vec![0,1],vec![1,0]])]
    #[case(vec![1], vec![vec![1]])]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: Vec<Vec<i32>>) {
        let canon = |mut v: Vec<Vec<i32>>| {
            v.sort();
            v
        };
        assert_eq!(canon(expected), canon(Solution::permute(nums)));
    }
}
