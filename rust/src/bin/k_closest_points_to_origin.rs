//! 973. K Closest Points to Origin
//! Medium | Array | Math | Divide and Conquer | Geometry | Sorting | Heap (Priority Queue) | Quickselect
//! https://leetcode.com/problems/k-closest-points-to-origin/
//!
//! Given an array of `points` where `points[i] = [x<sub>i</sub>,
//! y<sub>i</sub>]` represents a point on the **X-Y** plane and an integer `k`,
//! return the `k` closest points to the origin `(0, 0)`.
//!
//! The distance between two points on the **X-Y** plane is the Euclidean
//! distance (i.e., `√(x<sub>1</sub> - x<sub>2</sub>)<sup>2</sup> +
//! (y<sub>1</sub> - y<sub>2</sub>)<sup>2</sup>`).
//!
//! You may return the answer in **any order**. The answer is **guaranteed** to
//! be **unique** (except for the order that it is in).
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/03/03/closestplane1.jpg)
//!
//! ```
//! Input: points = [[1,3],[-2,2]], k = 1
//! Output: [[-2,2]]
//! Explanation:
//! The distance between (1, 3) and the origin is sqrt(10).
//! The distance between (-2, 2) and the origin is sqrt(8).
//! Since sqrt(8) < sqrt(10), (-2, 2) is closer to the origin.
//! We only want the closest k = 1 points from the origin, so the answer is just [[-2,2]].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: points = [[3,3],[5,-1],[-2,4]], k = 2
//! Output: [[3,3],[-2,4]]
//! Explanation: The answer [[-2,4],[3,3]] would also be accepted.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= k <= points.length <= 10<sup>4</sup>`
//! * `-10<sup>4</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>4</sup>`

struct Solution;

impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        points.select_nth_unstable_by_key(k - 1, |p| p[0] * p[0] + p[1] * p[1]);
        points.truncate(k);
        points
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3],vec![-2,2]], 1, vec![vec![-2,2]])]
    #[case(vec![vec![3,3],vec![5,-1],vec![-2,4]], 2, vec![vec![3,3],vec![-2,4]])]
    fn cases(#[case] points: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: Vec<Vec<i32>>) {
        assert_eq!(expected, Solution::k_closest(points, k));
    }
}
