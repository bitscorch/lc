//! 238. Product of Array Except Self
//! Medium | Array | Prefix Sum
//! https://leetcode.com/problems/product-of-array-except-self/
//!
//! Given an integer array `nums`, return *an array* `answer` *such that*
//! `answer[i]` *is equal to the product of all the elements of* `nums` *except*
//! `nums[i]`.
//!
//! The product of any prefix or suffix of `nums` is **guaranteed** to fit in a
//! **32-bit** integer.
//!
//! You must write an algorithm that runs in `O(n)` time and without using the
//! division operation.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,2,3,4]
//! Output: [24,12,8,6]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [-1,1,0,-3,3]
//! Output: [0,0,9,0,0]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `2 <= nums.length <= 10<sup>5</sup>`
//! * `-30 <= nums[i] <= 30`
//! * The input is generated such that `answer[i]` is **guaranteed** to fit in a **32-bit** integer.
//!
//! **Follow up:** Can you solve the problem in `O(1)` extra space complexity?
//! (The output array **does not** count as extra space for space complexity
//! analysis.)

struct Solution;

// if there is no 0 this is simple
// prod in var and then prod / num
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let zeroes = nums.iter().filter(|&&x| x == 0).count();
        if zeroes > 1 {
            return vec![0; nums.len()];
        }

        let prod: i32 = nums.iter().filter(|&&x| x != 0).product();

        nums.iter()
            .map(|&num| {
                if zeroes == 1 {
                    if num == 0 { prod } else { 0 }
                } else {
                    prod / num
                }
            })
            .collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4], vec![24,12,8,6])]
    #[case(vec![-1,1,0,-3,3], vec![0,0,9,0,0])]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        assert_eq!(expected, Solution::product_except_self(nums));
    }
}
