//! 543. Diameter of Binary Tree
//! Easy | Tree | Depth-First Search | Binary Tree
//! https://leetcode.com/problems/diameter-of-binary-tree/
//!
//! Given the `root` of a binary tree, return *the length of the **diameter** of
//! the tree*.
//!
//! The **diameter** of a binary tree is the **length** of the longest path
//! between any two nodes in a tree. This path may or may not pass through the
//! `root`.
//!
//! The **length** of a path between two nodes is represented by the number of
//! edges between them.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/03/06/diamtree.jpg)
//!
//! ```
//! Input: root = [1,2,3,4,5]
//! Output: 3
//! Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: root = [1,2]
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[1, 10<sup>4</sup>]`.
//! * `-100 <= Node.val <= 100`

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
use std::rc::Rc;

fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if let Some(node) = node {
        let (lh, ld) = dfs(node.borrow().left.clone());
        let (rh, rd) = dfs(node.borrow().right.clone());
        (1 + lh.max(rh), ld.max(rd).max(lh + rh))
    } else {
        (0, 0)
    }
}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(root).1
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::vec_to_tree;
    use rstest::rstest;

    #[rstest]
    #[case(vec![Some(1), Some(2), Some(3), Some(4), Some(5)], 3)]
    #[case(vec![Some(1), Some(2)], 1)]
    fn cases(#[case] root: Vec<Option<i32>>, #[case] expected: i32) {
        assert_eq!(
            expected,
            Solution::diameter_of_binary_tree(vec_to_tree(root))
        );
    }
}
