//! 217. Contains Duplicate
//! Easy | Array | Hash Table | Sorting
//! https://leetcode.com/problems/contains-duplicate/
//!
//! Given an integer array `nums`, return `true` if any value appears **at least
//! twice** in the array, and return `false` if every element is distinct.
//!
//! **Example 1:**
//!
//! **Input:** nums = [1,2,3,1]
//!
//! **Output:** true
//!
//! **Explanation:**
//!
//! The element 1 occurs at the indices 0 and 3.
//!
//! **Example 2:**
//!
//! **Input:** nums = [1,2,3,4]
//!
//! **Output:** false
//!
//! **Explanation:**
//!
//! All elements are distinct.
//!
//! **Example 3:**
//!
//! **Input:** nums = [1,1,1,3,3,4,3,2,4,2]
//!
//! **Output:** true
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10<sup>5</sup>`
//! * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut cache = HashSet::new();

        for num in nums {
            if cache.contains(&num) {
                return true;
            }
            cache.insert(num);
        }

        false
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,1], true)]
    #[case(vec![1,2,3,4], false)]
    #[case(vec![1,1,1,3,3,4,3,2,4,2], true)]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: bool) {
        assert_eq!(expected, Solution::contains_duplicate(nums));
    }
}
