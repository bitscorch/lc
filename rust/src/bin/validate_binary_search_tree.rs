//! 98. Validate Binary Search Tree
//! Medium | Tree | Depth-First Search | Binary Search Tree | Binary Tree
//! https://leetcode.com/problems/validate-binary-search-tree/
//!
//! Given the `root` of a binary tree, *determine if it is a valid binary search
//! tree (BST)*.
//!
//! A **valid BST** is defined as follows:
//!
//! * The left subtree of a node contains only nodes with keys **strictly less than** the node's key.
//! * The right subtree of a node contains only nodes with keys **strictly greater than** the node's key.
//! * Both the left and right subtrees must also be binary search trees.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/12/01/tree1.jpg)
//!
//! ```
//! Input: root = [2,1,3]
//! Output: true
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/12/01/tree2.jpg)
//!
//! ```
//! Input: root = [5,1,4,null,null,3,6]
//! Output: false
//! Explanation: The root node's value is 5 but its right child's value is 4.
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[1, 10<sup>4</sup>]`.
//! * `-2<sup>31</sup> <= Node.val <= 2<sup>31</sup> - 1`

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

fn check(node: &Option<Rc<RefCell<TreeNode>>>, lt: Option<i32>, gt: Option<i32>) -> bool {
    match node {
        None => true,
        Some(node) => {
            let node = node.borrow();
            lt.is_none_or(|lt| node.val < lt)
                && gt.is_none_or(|gt| node.val > gt)
                && check(&node.left, Some(node.val), gt)
                && check(&node.right, lt, Some(node.val))
        }
    }
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        check(&root, None, None)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::vec_to_tree;
    use rstest::rstest;

    #[rstest]
    #[case(vec![Some(2), Some(1), Some(3)], true)]
    #[case(vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)], false)]
    fn cases(#[case] root: Vec<Option<i32>>, #[case] expected: bool) {
        assert_eq!(expected, Solution::is_valid_bst(vec_to_tree(root)));
    }
}
