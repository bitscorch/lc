//! 3020. Find the Maximum Number of Elements in Subset
//! Medium | Array | Hash Table | Enumeration
//! https://leetcode.com/problems/find-the-maximum-number-of-elements-in-subset/
//!
//! You are given an array of **positive** integers `nums`.
//!
//! You need to select a subset of `nums` which satisfies the following
//! condition:
//!
//! * You can place the selected elements in a **0-indexed** array such that it follows the pattern: `[x, x<sup>2</sup>, x<sup>4</sup>, ..., x<sup>k/2</sup>, x<sup>k</sup>, x<sup>k/2</sup>, ..., x<sup>4</sup>, x<sup>2</sup>, x]` (**Note** that `k` can be be any **non-negative** power of `2`). For example, `[2, 4, 16, 4, 2]` and `[3, 9, 3]` follow the pattern while `[2, 4, 8, 4, 2]` does not.
//!
//! Return *the **maximum** number of elements in a subset that satisfies these
//! conditions.*
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [5,4,1,2,2]
//! Output: 3
//! Explanation: We can select the subset {4,2,2}, which can be placed in the array as [2,4,2] which follows the pattern and 22 == 4. Hence the answer is 3.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [1,3,2,4]
//! Output: 1
//! Explanation: We can select the subset {1}, which can be placed in the array as [1] which follows the pattern. Hence the answer is 1. Note that we could have also selected the subsets {2}, {3}, or {4}, there may be multiple subsets which provide the same answer.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `2 <= nums.length <= 10<sup>5</sup>`
//! * `1 <= nums[i] <= 10<sup>9</sup>`

struct Solution;

use std::collections::HashMap;

fn to_odd(n: i32) -> i32 {
    if n % 2 == 0 { n - 1 } else { n }
}

fn g(x: i32, counts: &HashMap<i32, i32>) -> i32 {
    let count = counts.get(&x).copied().unwrap_or(0);

    if count == 0 {
        return 0;
    }

    if count >= 2 {
        let sub = g(x * x, counts);
        if sub > 0 { 2 + sub } else { 1 }
    } else {
        1
    }
}

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        // counts hashmap, MAX (store hits if something hit, no way it's better)
        // sort
        // special case for 1 to make it fast potentiall (optional)
        // pow 2 (iterator) 2 -> 4 -> 8 -> 16 -> ...
        // while x ** pow < MAX && counts[x ** pow] >= 2 (count streak)
        // if coutns[x ** pow * 2] == 1 { streak + 1 }
        // go next
        // check counts hashmap for the longest chain
        //
        // 2 4 ... 16

        let mut counts: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut best = 0;
        for (&x, &count) in &counts {
            if x == 1 {
                best = best.max(to_odd(count));
                continue;
            }
            best = best.max(g(x, &counts));
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
    #[case(vec![5,4,1,2,2], 3)]
    #[case(vec![1,3,2,4], 1)]
    #[case(vec![1,1], 1)]
    #[case(vec![14,14,196,196,38416,38416], 5)]
    fn cases(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(expected, Solution::maximum_length(nums));
    }
}
