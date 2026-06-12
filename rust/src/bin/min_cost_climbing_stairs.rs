//! 746. Min Cost Climbing Stairs
//! Easy | Array | Dynamic Programming
//! https://leetcode.com/problems/min-cost-climbing-stairs/
//!
//! You are given an integer array `cost` where `cost[i]` is the cost of
//! `i<sup>th</sup>` step on a staircase. Once you pay the cost, you can either
//! climb one or two steps.
//!
//! You can either start from the step with index `0`, or the step with index
//! `1`.
//!
//! Return *the minimum cost to reach the top of the floor*.
//!
//! **Example 1:**
//!
//! ```
//! Input: cost = [10,15,20]
//! Output: 15
//! Explanation: You will start at index 1.
//! - Pay 15 and climb two steps to reach the top.
//! The total cost is 15.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: cost = [1,100,1,1,1,100,1,1,100,1]
//! Output: 6
//! Explanation: You will start at index 0.
//! - Pay 1 and climb two steps to reach index 2.
//! - Pay 1 and climb two steps to reach index 4.
//! - Pay 1 and climb two steps to reach index 6.
//! - Pay 1 and climb one step to reach index 7.
//! - Pay 1 and climb two steps to reach index 9.
//! - Pay 1 and climb one step to reach the top.
//! The total cost is 6.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `2 <= cost.length <= 1000`
//! * `0 <= cost[i] <= 999`

struct Solution;

// [a, b, c, d]
// a > b > c > d > end
// a > c > d > end
// a > c > end
// b > c > d > end
// b > d > end
//
// min cost to reach a node
// [a, b, c, d, e, f]
// a -> 0
// b -> 0
// c -> min(a, b)
// d -> min(c, b)
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut a, mut b) = (cost[0], cost[1]);

        for &c in &cost[2..] {
            (a, b) = (b, c + a.min(b));
        }

        a.min(b)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Inputs are filled from LeetCode's examples; replace each `0` with the
    // expected output (see the `Output:` lines in the description above).
    #[test]
    fn case_1() {
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
}
