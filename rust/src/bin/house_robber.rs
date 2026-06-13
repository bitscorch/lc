//! 198. House Robber
//! Medium | Array | Dynamic Programming
//! https://leetcode.com/problems/house-robber/
//!
//! You are a professional robber planning to rob houses along a street. Each
//! house has a certain amount of money stashed, the only constraint stopping
//! you from robbing each of them is that adjacent houses have security systems
//! connected and **it will automatically contact the police if two adjacent
//! houses were broken into on the same night**.
//!
//! Given an integer array `nums` representing the amount of money of each
//! house, return *the maximum amount of money you can rob tonight **without
//! alerting the police***.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,2,3,1]
//! Output: 4
//! Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//! Total amount you can rob = 1 + 3 = 4.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [2,7,9,3,1]
//! Output: 12
//! Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
//! Total amount you can rob = 2 + 9 + 1 = 12.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 100`
//! * `0 <= nums[i] <= 400`

struct Solution;

// max money robabble from __ => f
// [1, 2, 3, 1]
// f(1) => [1] => 1
// f(2) => [1, 2] => 2
// f(3) => [1, 2, 3] => 4
// f(4) => [1, 2, 3, 1] => 4
//
// f(4) => 1, 2, 3, 1 => rob: f(2) + 1 || skip: f(3)
//                  ^
// f(4) = max(1 + f(2), f(3))
// f(3) = max(3 + f(1), f(2))
// f(2) = max(1, 2)
// f(1) = 1
//
// nums = [2, 7, 9, 3, 1]
// f(0) = 0                   => a
// f(1) = max(0, 2) => 2      => b
// f(2) = max(7 + f(0), f(1)) =>
// f(3) = max(9 + f(1), f(2))
// f(4) = max(3 + f(2), f(3))
// f(5) = max(1 + f(3), f(4))
//
// f(i) = max(nums[i - 1] + f(i - 2), f(i - 1))
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, nums[0]);
        for &n in &nums[1..] {
            (a, b) = (b, b.max(n + a));
        }
        b
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 2, 3, 1], 4)]
    #[case(vec![2, 7, 9, 3, 1], 12)]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(expected, Solution::rob(nums));
    }
}
