//! 278. First Bad Version
//! Easy | Binary Search | Interactive
//! https://leetcode.com/problems/first-bad-version/
//!
//! You are a product manager and currently leading a team to develop a new
//! product. Unfortunately, the latest version of your product fails the quality
//! check. Since each version is developed based on the previous version, all
//! the versions after a bad version are also bad.
//!
//! Suppose you have `n` versions `[1, 2, ..., n]` and you want to find out the
//! first bad one, which causes all the following ones to be bad.
//!
//! You are given an API `bool isBadVersion(version)` which returns whether
//! `version` is bad. Implement a function to find the first bad version. You
//! should minimize the number of calls to the API.
//!
//! **Example 1:**
//!
//! ```
//! Input: n = 5, bad = 4
//! Output: 4
//! Explanation:
//! call isBadVersion(3) -> false
//! call isBadVersion(5) -> true
//! call isBadVersion(4) -> true
//! Then 4 is the first bad version.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: n = 1, bad = 1
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= bad <= n <= 2<sup>31</sup> - 1`

// ---- local-only stub of LeetCode's hidden isBadVersion API ----
// Above `struct Solution;` so `lc submit` strips it. Each test thread gets its
// own FIRST_BAD (thread_local), so parallel test cases don't clobber each other.
use std::cell::Cell;
thread_local!(static FIRST_BAD: Cell<i32> = const { Cell::new(1) });

impl Solution {
    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        version >= FIRST_BAD.with(Cell::get)
    }
}

struct Solution;

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut l, mut r) = (1, n);
        while l < r {
            let mid = l + (r - l) / 2;
            if self.isBadVersion(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(5, 4)] // versions 4,5 bad → first bad is 4
    #[case(1, 1)] // single version, and it's bad
    #[case(10, 1)] // everything is bad
    #[case(10, 10)] // only the last is bad
    fn cases(#[case] n: i32, #[case] first_bad: i32) {
        FIRST_BAD.with(|b| b.set(first_bad));
        assert_eq!(first_bad, Solution.first_bad_version(n));
    }
}
