//! 232. Implement Queue using Stacks
//! Easy | Stack | Design | Queue
//! https://leetcode.com/problems/implement-queue-using-stacks/
//!
//! Implement a first in first out (FIFO) queue using only two stacks. The
//! implemented queue should support all the functions of a normal queue
//! (`push`, `peek`, `pop`, and `empty`).
//!
//! Implement the `MyQueue` class:
//!
//! * `void push(int x)` Pushes element x to the back of the queue.
//! * `int pop()` Removes the element from the front of the queue and returns it.
//! * `int peek()` Returns the element at the front of the queue.
//! * `boolean empty()` Returns `true` if the queue is empty, `false` otherwise.
//!
//! **Notes:**
//!
//! * You must use **only** standard operations of a stack, which means only `push to top`, `peek/pop from top`, `size`, and `is empty` operations are valid.
//! * Depending on your language, the stack may not be supported natively. You may simulate a stack using a list or deque (double-ended queue) as long as you use only a stack's standard operations.
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["MyQueue", "push", "push", "peek", "pop", "empty"]
//! [[], [1], [2], [], [], []]
//! Output
//! [null, null, null, 1, 1, false]
//!
//! Explanation
//! MyQueue myQueue = new MyQueue();
//! myQueue.push(1); // queue is: [1]
//! myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
//! myQueue.peek(); // return 1
//! myQueue.pop(); // return 1, queue is [2]
//! myQueue.empty(); // return false
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= x <= 9`
//! * At most `100` calls will be made to `push`, `pop`, `peek`, and `empty`.
//! * All the calls to `pop` and `peek` are valid.
//!
//! **Follow-up:** Can you implement the queue such that each operation is
//! **[amortized](https://en.wikipedia.org/wiki/Amortized_analysis)** `O(1)`
//! time complexity? In other words, performing `n` operations will take overall
//! `O(n)` time even if one of those operations may take longer.

struct Solution;

struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

// [a, b, c, d, x]
// [d, c, b, a]
impl MyQueue {
    fn new() -> Self {
        Self {
            in_stack: vec![],
            out_stack: vec![],
        }
    }

    fn pour(&mut self) {
        self.out_stack.extend(self.in_stack.drain(..).rev())
    }

    fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.pour();
        self.out_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.pour();
        *self.out_stack.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Design problems are tested as an operation sequence, not parametrized
    // cases. This is LeetCode's worked example:
    //   ["MyQueue","push","push","peek","pop","empty"]
    //   [[], [1], [2], [], [], []]  ->  [null, null, null, 1, 1, false]
    #[test]
    fn example() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
        // drain the rest: FIFO means 2 comes out next, then it's empty
        assert_eq!(q.pop(), 2);
        assert!(q.empty());
    }
}
