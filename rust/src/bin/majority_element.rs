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

// [a, b, c, a, a, a]
// a 1 0
// a 0
// c 1
// c 0
// a 1
// a 2
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut ans, mut cnt) = (0, 0);

        for num in nums {
            if cnt == 0 {
                ans = num;
                cnt += 1;
            } else if ans == num {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }

        ans
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
