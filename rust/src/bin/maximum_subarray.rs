//! 53. Maximum Subarray
//! Medium | Array | Divide and Conquer | Dynamic Programming
//! https://leetcode.com/problems/maximum-subarray/
//!
//! Given an integer array `nums`, find the subarray with the largest sum, and
//! return *its sum*.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
//! Output: 6
//! Explanation: The subarray [4,-1,2,1] has the largest sum 6.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [1]
//! Output: 1
//! Explanation: The subarray [1] has the largest sum 1.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [5,4,-1,7,8]
//! Output: 23
//! Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10<sup>5</sup>`
//! * `-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>`
//!
//! **Follow up:** If you have figured out the `O(n)` solution, try coding another solution using the **divide and conquer** approach, which is more subtle.

struct Solution;

// Did we find a better subarray at `i`?
// best(i) vs best(i-1)
// [-2, 1, -3, 4, -1, 2, 1, -5, 4]
// 0: -2
// 1: -1,  1 => 1
// 2: -2, -3 => -2
// 3:  2,  4 => 4
// 4:  3, -1 => 3
// 5:  5,  2 => 5
// 6:  6,  1 => 6
// 7:  1, -5 => 1
// 8:  5,  4 => 5
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut cur, mut best) = (nums[0], nums[0]);

        for &x in &nums[1..] {
            cur = x.max(cur + x);
            best = best.max(cur);
        }

        best
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6)]
    #[case(vec![1], 1)]
    #[case(vec![5, 4, -1, 7, 8], 23)]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(expected, Solution::max_sub_array(nums));
    }
}
