//! 15. 3Sum
//! Medium | Array | Two Pointers | Sorting
//! https://leetcode.com/problems/3sum/
//!
//! Given an integer array nums, return all the triplets `[nums[i], nums[j],
//! nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j]
//! + nums[k] == 0`.
//!
//! Notice that the solution set must not contain duplicate triplets.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [-1,0,1,2,-1,-4]
//! Output: [[-1,-1,2],[-1,0,1]]
//! Explanation:
//! nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
//! nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
//! nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
//! The distinct triplets are [-1,0,1] and [-1,-1,2].
//! Notice that the order of the output and the order of the triplets does not matter.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [0,1,1]
//! Output: []
//! Explanation: The only possible triplet does not sum up to 0.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [0,0,0]
//! Output: [[0,0,0]]
//! Explanation: The only possible triplet sums up to 0.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `3 <= nums.length <= 3000`
//! * `-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup>`

struct Solution;

// 3 point solution???
// HM with 2 pointer???
// [-4, -1, -1, 0, 1, 2]
//   ^                ^
// (-2) if sum < 0 right to left
// -1 if new sum < 0
// ( 1) if sum > 0 left to right
//
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ans = Vec::with_capacity(nums.len());

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut p1, mut p2) = (i + 1, nums.len() - 1);

            while p1 < p2 {
                if nums[p1] + nums[p2] + nums[i] > 0 {
                    p2 -= 1;
                    while p1 < p2 && nums[p2] == nums[p2 + 1] {
                        p2 -= 1
                    }
                } else if nums[p1] + nums[p2] + nums[i] < 0 {
                    p1 += 1;
                    while p1 < p2 && nums[p1] == nums[p1 - 1] {
                        p1 += 1
                    }
                } else {
                    ans.push(vec![nums[i], nums[p1], nums[p2]]);
                    p1 += 1;
                    while p1 < p2 && nums[p1] == nums[p1 - 1] {
                        p1 += 1
                    }
                    p2 -= 1;
                    while p1 < p2 && nums[p2] == nums[p2 + 1] {
                        p2 -= 1
                    }
                }
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
    #[case(vec![-1,0,1,2,-1,-4], vec![vec![-1,-1,2],vec![-1,0,1]])]
    #[case(vec![0,1,1], vec![])]
    #[case(vec![0,0,0], vec![vec![0,0,0]])]
    #[case(vec![0,0,0,0], vec![vec![0,0,0]])]
    #[case(vec![-100,-70,-60,110,120,130,160], vec![vec![-100,-60,160],vec![-70,-60,130]])]
    #[case(vec![1,2,0,1,0,0,0,0], vec![vec![0,0,0]])]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: Vec<Vec<i32>>) {
        assert_eq!(expected, Solution::three_sum(nums));
    }
}
