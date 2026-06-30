//! 102. Binary Tree Level Order Traversal
//! Medium | Tree | Breadth-First Search | Binary Tree
//! https://leetcode.com/problems/binary-tree-level-order-traversal/
//!
//! Given the `root` of a binary tree, return *the level order traversal of its
//! nodes' values*. (i.e., from left to right, level by level).
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg)
//!
//! ```
//! Input: root = [3,9,20,null,null,15,7]
//! Output: [[3],[9,20],[15,7]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: root = [1]
//! Output: [[1]]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: root = []
//! Output: []
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[0, 2000]`.
//! * `-1000 <= Node.val <= 1000`

use lc::TreeNode;

struct Solution;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// node
// node -> val
// r => node
// l r
// l r l r
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut ans = vec![];
        let mut level = VecDeque::new();
        level.push_back(root.unwrap());

        while !level.is_empty() {
            let mut n_level = VecDeque::new();
            let mut n_row = vec![];
            while let Some(node) = level.pop_front() {
                let n = node.borrow();
                n_row.push(n.val);
                if let Some(left) = n.left.clone() {
                    n_level.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    n_level.push_back(right);
                }
            }
            ans.push(n_row);
            level = n_level;
        }

        ans
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::vec_to_tree;
    use rstest::rstest;

    #[rstest]
    #[case(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)], vec![vec![3],vec![9,20],vec![15,7]])]
    #[case(vec![Some(1)], vec![vec![1]])]
    #[case(vec![], vec![])]
    #[case(vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5)], vec![vec![1], vec![2, 3], vec![4, 5]])]
    fn cases(#[case] root: Vec<Option<i32>>, #[case] expected: Vec<Vec<i32>>) {
        assert_eq!(expected, Solution::level_order(vec_to_tree(root)));
    }
}
