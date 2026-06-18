//! Shared helpers for LeetCode solutions that use custom types (linked lists,
//! binary trees). LeetCode's judge defines `ListNode`/`TreeNode` itself, so the
//! `use lc::...` line is stripped from submissions — these definitions exist
//! purely so solutions compile and tests run locally.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Singly-linked list node, matching LeetCode's definition.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Build a linked list from a vector (the head is the first element).
pub fn vec_to_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in values.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

/// Collect a linked list back into a vector — handy for comparing in tests.
pub fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(n) = node {
        out.push(n.val);
        node = n.next;
    }
    out
}

/// Binary tree node, matching LeetCode's definition.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type Tree = Option<Rc<RefCell<TreeNode>>>;

/// Build a tree from LeetCode's level-order array (`None` = a missing node).
pub fn vec_to_tree(values: Vec<Option<i32>>) -> Tree {
    let mut it = values.into_iter();
    let root = match it.next() {
        Some(Some(v)) => Rc::new(RefCell::new(TreeNode::new(v))),
        _ => return None,
    };
    let mut queue = VecDeque::from([root.clone()]);
    while let Some(node) = queue.pop_front() {
        if let Some(Some(v)) = it.next() {
            let child = Rc::new(RefCell::new(TreeNode::new(v)));
            node.borrow_mut().left = Some(child.clone());
            queue.push_back(child);
        }
        if let Some(Some(v)) = it.next() {
            let child = Rc::new(RefCell::new(TreeNode::new(v)));
            node.borrow_mut().right = Some(child.clone());
            queue.push_back(child);
        }
    }
    Some(root)
}

/// Serialize a tree back to LeetCode's level-order array, trailing nulls
/// trimmed — so it compares equal to the `Output:` arrays in tests.
pub fn tree_to_vec(root: Tree) -> Vec<Option<i32>> {
    let mut out = Vec::new();
    let mut queue = VecDeque::from([root]);
    while let Some(node) = queue.pop_front() {
        match node {
            Some(n) => {
                let n = n.borrow();
                out.push(Some(n.val));
                queue.push_back(n.left.clone());
                queue.push_back(n.right.clone());
            }
            None => out.push(None),
        }
    }
    while out.last() == Some(&None) {
        out.pop();
    }
    out
}
