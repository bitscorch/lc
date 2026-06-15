//! Shared helpers for LeetCode solutions that use custom types (linked lists,
//! etc.). LeetCode's judge defines `ListNode` itself, so `lc submit` strips the
//! `use lc::*;` line from submissions — these definitions exist purely so
//! solutions compile and tests run locally.

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
