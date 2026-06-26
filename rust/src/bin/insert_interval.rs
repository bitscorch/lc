//! 57. Insert Interval
//! Medium | Array
//! https://leetcode.com/problems/insert-interval/
//!
//! You are given an array of non-overlapping intervals `intervals` where
//! `intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]` represent the start
//! and the end of the `i<sup>th</sup>` interval and `intervals` is sorted in
//! ascending order by `start<sub>i</sub>`. You are also given an interval
//! `newInterval = [start, end]` that represents the start and end of another
//! interval.
//!
//! Insert `newInterval` into `intervals` such that `intervals` is still sorted
//! in ascending order by `start<sub>i</sub>` and `intervals` still does not
//! have any overlapping intervals (merge overlapping intervals if necessary).
//!
//! Return `intervals` *after the insertion*.
//!
//! **Note** that you don't need to modify `intervals` in-place. You can make a
//! new array and return it.
//!
//! **Example 1:**
//!
//! ```
//! Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
//! Output: [[1,5],[6,9]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
//! Output: [[1,2],[3,10],[12,16]]
//! Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= intervals.length <= 10<sup>4</sup>`
//! * `intervals[i].length == 2`
//! * `0 <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>5</sup>`
//! * `intervals` is sorted by `start<sub>i</sub>` in **ascending** order.
//! * `newInterval.length == 2`
//! * `0 <= start <= end <= 10<sup>5</sup>`

struct Solution;

// A--B C--D E--F G--H I--J
//        L--------R
//
// C <= L && D >= L
// E >= L && F <= R
// G <= R && H >= R
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut l, mut r) = (new_interval[0], new_interval[1]);
        let mut ans = Vec::with_capacity(intervals.len() + 1);
        let mut it = intervals.into_iter().peekable();

        while it.peek().is_some_and(|iv| iv[1] < l) {
            ans.push(it.next().unwrap());
        }
        while it.peek().is_some_and(|iv| iv[0] <= r) {
            let iv = it.next().unwrap();
            l = l.min(iv[0]);
            r = r.max(iv[1]);
        }
        ans.push(vec![l, r]);
        ans.extend(it);
        ans
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3],vec![6,9]], vec![2,5], vec![vec![1,5],vec![6,9]])]
    #[case(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4,8], vec![vec![1,2],vec![3,10],vec![12,16]])]
    fn cases(
        #[case] intervals: Vec<Vec<i32>>,
        #[case] new_interval: Vec<i32>,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }
}
