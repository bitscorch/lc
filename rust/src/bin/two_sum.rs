//! 1. Two Sum
//! Easy | Array | Hash Table
//! https://leetcode.com/problems/two-sum/
//!
//! Given an array of integers `nums` and an integer `target`, return *indices
//! of the two numbers such that they add up to `target`*.
//!
//! You may assume that each input would have ***exactly* one solution**, and
//! you may not use the *same* element twice.
//!
//! You can return the answer in any order.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [2,7,11,15], target = 9
//! Output: [0,1]
//! Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [3,2,4], target = 6
//! Output: [1,2]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [3,3], target = 6
//! Output: [0,1]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `2 <= nums.length <= 10<sup>4</sup>`
//! * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
//! * `-10<sup>9</sup> <= target <= 10<sup>9</sup>`
//! * **Only one valid answer exists.**
//!
//! **Follow-up:** Can you come up with an algorithm that is less than `O(n<sup>2</sup>)` time complexity?

struct Solution;

use std::collections::HashMap;

// HashMap?
// for x in nums
// {
// if x
// }
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            if let Some(j) = hm.get(x) {
                return vec![*j, i as i32];
            } else {
                hm.insert(target - x, i as i32);
            }
        }
        vec![]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2,7,11,15], 9, vec![0,1])]
    #[case(vec![3,2,4], 6, vec![1,2])]
    #[case(vec![3,3], 6, vec![0,1])]
    fn cases(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        assert_eq!(expected, Solution::two_sum(nums, target));
    }
}
