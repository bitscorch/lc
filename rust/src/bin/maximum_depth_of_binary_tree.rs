//! 104. Maximum Depth of Binary Tree
//! Easy | Tree | Depth-First Search | Breadth-First Search | Binary Tree
//! https://leetcode.com/problems/maximum-depth-of-binary-tree/
//!
//! Given the `root` of a binary tree, return *its maximum depth*.
//!
//! A binary tree's **maximum depth** is the number of nodes along the longest
//! path from the root node down to the farthest leaf node.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/11/26/tmp-tree.jpg)
//!
//! ```
//! Input: root = [3,9,20,null,null,15,7]
//! Output: 3
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: root = [1,null,2]
//! Output: 2
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[0, 10<sup>4</sup>]`.
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

fn depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = node {
        let n = node.borrow();
        1 + depth(&n.left).max(depth(&n.right))
    } else {
        0
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        depth(&root)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::vec_to_tree;
    use rstest::rstest;

    #[rstest]
    #[case(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)], 3)]
    #[case(vec![Some(1), None, Some(2)], 2)]
    fn cases(#[case] root: Vec<Option<i32>>, #[case] expected: i32) {
        assert_eq!(expected, Solution::max_depth(vec_to_tree(root)));
    }
}
