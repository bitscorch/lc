//! 169. Majority Element
//! Easy | Array | Hash Table | Divide and Conquer | Sorting | Counting
//! https://leetcode.com/problems/majority-element/
//!
//! Given an array `nums` of size `n`, return *the majority element*.
//!
//! The majority element is the element that appears more than `⌊n / 2⌋`
//! times. You may assume that the majority element always exists in the array.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [3,2,3]
//! Output: 3
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [2,2,1,1,1,2,2]
//! Output: 2
//!
//! ```
//!
//! **Constraints:**
//!
//! * `n == nums.length`
//! * `1 <= n <= 5 * 10<sup>4</sup>`
//! * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
//! * The input is generated such that a majority element will exist in the array.
//!
//! **Follow-up:** Could you solve the problem in linear time and in `O(1)`
//! space?

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnts = HashMap::new();
        let n = nums.len();

        for num in nums {
            let cnt = cnts.entry(num).or_insert(0);
            *cnt += 1;
            if *cnt > (n / 2) as i32 {
                return num;
            }
        }

        unreachable!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,3], 3)]
    #[case(vec![2,2,1,1,1,2,2], 2)]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(expected, Solution::majority_element(nums));
    }
}
