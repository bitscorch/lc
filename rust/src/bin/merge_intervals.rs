//! 56. Merge Intervals
//! Medium | Array | Sorting | Quicksort
//! https://leetcode.com/problems/merge-intervals/
//!
//! Given an array of `intervals` where `intervals[i] = [start<sub>i</sub>,
//! end<sub>i</sub>]`, merge all overlapping intervals, and return *an array of
//! the non-overlapping intervals that cover all the intervals in the input*.
//!
//! **Example 1:**
//!
//! ```
//! Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
//! Output: [[1,6],[8,10],[15,18]]
//! Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: intervals = [[1,4],[4,5]]
//! Output: [[1,5]]
//! Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: intervals = [[4,7],[1,4]]
//! Output: [[1,7]]
//! Explanation: Intervals [1,4] and [4,7] are considered overlapping.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= intervals.length <= 10<sup>4</sup>`
//! * `intervals[i].length == 2`
//! * `0 <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>4</sup>`

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        intervals.sort_by_key(|x| x[0]);
        let mut last_interval = intervals[0].clone();

        for interval in &intervals[1..] {
            if last_interval[1] >= interval[0] {
                last_interval[1] = interval[1].max(last_interval[1]);
            } else {
                res.push(last_interval);
                last_interval = interval.clone();
            }
        }

        res.push(last_interval);
        res
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]], vec![vec![1,6],vec![8,10],vec![15,18]])]
    #[case(vec![vec![1,4],vec![4,5]], vec![vec![1,5]])]
    #[case(vec![vec![4,7],vec![1,4]], vec![vec![1,7]])]
    #[case(vec![vec![1,4],vec![0,4]], vec![vec![0,4]])]
    #[case(vec![vec![1,4],vec![2,3]], vec![vec![1,4]])]
    #[case(vec![vec![2,3],vec![4,5],vec![6,7],vec![8,9],vec![1,10]], vec![vec![1,10]])]
    fn cases(#[case] intervals: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        assert_eq!(expected, Solution::merge(intervals));
    }
}
