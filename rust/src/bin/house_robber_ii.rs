//! 213. House Robber II
//! Medium | Array | Dynamic Programming
//! https://leetcode.com/problems/house-robber-ii/
//!
//! You are a professional robber planning to rob houses along a street. Each
//! house has a certain amount of money stashed. All houses at this place are
//! **arranged in a circle.** That means the first house is the neighbor of the
//! last one. Meanwhile, adjacent houses have a security system connected, and
//! **it will automatically contact the police if two adjacent houses were
//! broken into on the same night**.
//!
//! Given an integer array `nums` representing the amount of money of each
//! house, return *the maximum amount of money you can rob tonight **without
//! alerting the police***.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [2,3,2]
//! Output: 3
//! Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [1,2,3,1]
//! Output: 4
//! Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//! Total amount you can rob = 1 + 3 = 4.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [1,2,3]
//! Output: 3
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 100`
//! * `0 <= nums[i] <= 1000`

struct Solution;

// nums = 1, 2, 3, 1
//  1 [0] - 2 [1]
// /         \
// 1 [3] --- 3 [2]
//
// 1[0]: 2, 3, 1 => 2 + 1 || 1 + 3
// 1[3]: 1, 2, 3 => 1 + 3 || 1 + 2
//
// nums 1, 2, 3, 1, 5
// make a line (house-robber)
// 2, 3, 1, 5 => 8
// 1, 2, 3, 1 => 4

fn rob_line(nums: &[i32]) -> i32 {
    nums.iter()
        .fold((0, 0), |(prev, cur), &n| (cur, cur.max(prev + n)))
        .1
}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums[0];
        }

        rob_line(&nums[1..]).max(rob_line(&nums[..nums.len() - 1]))
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2, 3, 2], 3)]
    #[case(vec![1, 2, 3, 1], 4)]
    #[case(vec![1, 2, 3], 3)]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(expected, Solution::rob(nums));
    }
}
