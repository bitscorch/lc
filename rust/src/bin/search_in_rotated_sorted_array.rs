//! 33. Search in Rotated Sorted Array
//! Medium | Array | Binary Search
//! https://leetcode.com/problems/search-in-rotated-sorted-array/
//!
//! There is an integer array `nums` sorted in ascending order (with
//! **distinct** values).
//!
//! Prior to being passed to your function, `nums` is **possibly left rotated**
//! at an unknown index `k` (`1 <= k < nums.length`) such that the resulting
//! array is `[nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ...,
//! nums[k-1]]` (**0-indexed**). For example, `[0,1,2,4,5,6,7]` might be left
//! rotated by `3` indices and become `[4,5,6,7,0,1,2]`.
//!
//! Given the array `nums` **after** the possible rotation and an integer
//! `target`, return *the index of* `target` *if it is in* `nums`*, or* `-1` *if
//! it is not in* `nums`.
//!
//! You must write an algorithm with `O(log n)` runtime complexity.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [4,5,6,7,0,1,2], target = 0
//! Output: 4
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [4,5,6,7,0,1,2], target = 3
//! Output: -1
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [1], target = 0
//! Output: -1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 5000`
//! * `-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>`
//! * All values of `nums` are **unique**.
//! * `nums` is an ascending array that is possibly rotated.
//! * `-10<sup>4</sup> <= target <= 10<sup>4</sup>`

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[lo] <= nums[mid] {
                if nums[lo] <= target && target < nums[mid] {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[hi - 1] {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
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
    #[case(vec![4,5,6,7,0,1,2], 0, 4)]
    #[case(vec![4,5,6,7,0,1,2], 3, -1)]
    #[case(vec![1], 0, -1)]
    #[case(vec![2], 2, 0)]
    #[case(vec![1, 2, 3], 2, 1)]
    fn cases(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        assert_eq!(expected, Solution::search(nums, target));
    }
}
