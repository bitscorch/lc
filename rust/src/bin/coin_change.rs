//! 322. Coin Change
//! Medium | Array | Dynamic Programming | Breadth-First Search
//! https://leetcode.com/problems/coin-change/
//!
//! You are given an integer array `coins` representing coins of different
//! denominations and an integer `amount` representing a total amount of money.
//!
//! Return *the fewest number of coins that you need to make up that amount*. If
//! that amount of money cannot be made up by any combination of the coins,
//! return `-1`.
//!
//! You may assume that you have an infinite number of each kind of coin.
//!
//! **Example 1:**
//!
//! ```
//! Input: coins = [1,2,5], amount = 11
//! Output: 3
//! Explanation: 11 = 5 + 5 + 1
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: coins = [2], amount = 3
//! Output: -1
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: coins = [1], amount = 0
//! Output: 0
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= coins.length <= 12`
//! * `1 <= coins[i] <= 2<sup>31</sup> - 1`
//! * `0 <= amount <= 10<sup>4</sup>`

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![amount + 1; amount + 1];
        dp[0] = 0;

        for i in 1..=amount {
            for &coin in &coins {
                let coin = coin as usize;
                if coin <= i {
                    dp[i] = dp[i].min(dp[i - coin] + 1);
                }
            }
        }

        if dp[amount] > amount {
            -1
        } else {
            dp[amount] as i32
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,5], 11, 3)]
    #[case(vec![2], 3, -1)]
    #[case(vec![1], 0, 0)]
    fn cases(#[case] coins: Vec<i32>, #[case] amount: i32, #[case] expected: i32) {
        assert_eq!(expected, Solution::coin_change(coins, amount));
    }
}
