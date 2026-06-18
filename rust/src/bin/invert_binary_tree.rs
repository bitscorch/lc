//! 226. Invert Binary Tree
//! Easy | Tree | Depth-First Search | Breadth-First Search | Binary Tree
//! https://leetcode.com/problems/invert-binary-tree/
//!
//! Given the `root` of a binary tree, invert the tree, and return *its root*.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/03/14/invert1-tree.jpg)
//!
//! ```
//! Input: root = [4,2,7,1,3,6,9]
//! Output: [4,7,2,9,6,3,1]
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/03/14/invert2-tree.jpg)
//!
//! ```
//! Input: root = [2,1,3]
//! Output: [2,3,1]
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
//! * The number of nodes in the tree is in the range `[0, 100]`.
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
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let mut guard = node.borrow_mut();
            let n = &mut *guard;
            std::mem::swap(&mut n.left, &mut n.right);
            Self::invert_tree(n.left.clone());
            Self::invert_tree(n.right.clone());
        };
        root
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::{tree_to_vec, vec_to_tree};
    use rstest::rstest;

    #[rstest]
    #[case(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)], vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)])]
    #[case(vec![Some(2), Some(1), Some(3)], vec![Some(2), Some(3), Some(1)])]
    #[case(vec![], vec![])]
    fn cases(#[case] root: Vec<Option<i32>>, #[case] expected: Vec<Option<i32>>) {
        assert_eq!(
            expected,
            tree_to_vec(Solution::invert_tree(vec_to_tree(root)))
        );
    }
}
