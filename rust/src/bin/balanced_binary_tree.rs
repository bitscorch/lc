//! 110. Balanced Binary Tree
//! Easy | Tree | Depth-First Search | Binary Tree
//! https://leetcode.com/problems/balanced-binary-tree/
//!
//! Given a binary tree, determine if it is **height-balanced**.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/06/balance_1.jpg)
//!
//! ```
//! Input: root = [3,9,20,null,null,15,7]
//! Output: true
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/06/balance_2.jpg)
//!
//! ```
//! Input: root = [1,2,2,3,3,null,null,4,4]
//! Output: false
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: root = []
//! Output: true
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[0, 5000]`.
//! * `-10<sup>4</sup> <= Node.val <= 10<sup>4</sup>`

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
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut order = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while let Some(node) = queue.pop_front() {
            let Some(node) = node else { continue };
            let n = node.borrow();
            queue.push_back(n.left.clone());
            queue.push_back(n.right.clone());
            drop(n);
            order.push(node);
        }

        let mut height = HashMap::<usize, i32>::new();
        for node in order.into_iter().rev() {
            let n = node.borrow();
            let hl = n
                .left
                .as_ref()
                .map_or(0, |c| height[&(Rc::as_ptr(c) as usize)]);
            let hr = n
                .right
                .as_ref()
                .map_or(0, |c| height[&(Rc::as_ptr(c) as usize)]);
            if (hl - hr).abs() > 1 {
                return false;
            }
            height.insert(Rc::as_ptr(&node) as usize, hl.max(hr) + 1);
        }
        true
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::vec_to_tree;
    use rstest::rstest;

    #[rstest]
    #[case(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)], true)]
    #[case(vec![Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4)], false)]
    #[case(vec![], true)]
    fn cases(#[case] root: Vec<Option<i32>>, #[case] expected: bool) {
        assert_eq!(expected, Solution::is_balanced(vec_to_tree(root)));
    }
}
