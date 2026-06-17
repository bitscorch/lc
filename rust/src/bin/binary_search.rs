//! 704. Binary Search
//! Easy | Array | Binary Search
//! https://leetcode.com/problems/binary-search/
//!
//! Given an array of integers `nums` which is sorted in ascending order, and an
//! integer `target`, write a function to search `target` in `nums`. If `target`
//! exists, then return its index. Otherwise, return `-1`.
//!
//! You must write an algorithm with `O(log n)` runtime complexity.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [-1,0,3,5,9,12], target = 9
//! Output: 4
//! Explanation: 9 exists in nums and its index is 4
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [-1,0,3,5,9,12], target = 2
//! Output: -1
//! Explanation: 2 does not exist in nums so return -1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10<sup>4</sup>`
//! * `-10<sup>4</sup> < nums[i], target < 10<sup>4</sup>`
//! * All the integers in `nums` are **unique**.
//! * `nums` is sorted in ascending order.

struct Solution;

// t
// [a b c d e f g]
//        ^
// [a b c d e f]
//      ^
// TODO: check
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut h) = (0, nums.len());

        while l != h {
            let m = (h + l) / 2;
            if target == nums[m] {
                return m as i32;
            } else if target > nums[m] {
                l = m + 1;
            } else {
                h = m;
            };
        }

        -1
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![-1,0,3,5,9,12], 9, 4)]
    #[case(vec![-1,0,3,5,9,12], 2, -1)]
    fn cases(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        assert_eq!(expected, Solution::search(nums, target));
    }
}
