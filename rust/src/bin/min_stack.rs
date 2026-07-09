//! 155. Min Stack
//! Medium | Stack | Design
//! https://leetcode.com/problems/min-stack/
//!
//! Design a stack that supports push, pop, top, and retrieving the minimum
//! element in constant time.
//!
//! Implement the `MinStack` class:
//!
//! * `MinStack()` initializes the stack object.
//! * `void push(int value)` pushes the element `value` onto the stack.
//! * `void pop()` removes the element on the top of the stack.
//! * `int top()` gets the top element of the stack.
//! * `int getMin()` retrieves the minimum element in the stack.
//!
//! You must implement a solution with `O(1)` time complexity for each function.
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["MinStack","push","push","push","getMin","pop","top","getMin"]
//! [[],[-2],[0],[-3],[],[],[],[]]
//!
//! Output
//! [null,null,null,null,-3,null,0,-2]
//!
//! Explanation
//! MinStack minStack = new MinStack();
//! minStack.push(-2);
//! minStack.push(0);
//! minStack.push(-3);
//! minStack.getMin(); // return -3
//! minStack.pop();
//! minStack.top();    // return 0
//! minStack.getMin(); // return -2
//!
//! ```
//!
//! **Constraints:**
//!
//! * `-2<sup>31</sup> <= val <= 2<sup>31</sup> - 1`
//! * Methods `pop`, `top` and `getMin` operations will always be called on **non-empty** stacks.
//! * At most `3 * 10<sup>4</sup>` calls will be made to `push`, `pop`, `top`, and `getMin`.

struct Solution;

struct MinStack {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn push(&mut self, value: i32) {
        if let Some(&(_, min)) = self.stack.last() {
            self.stack.push((value, min.min(value)));
        } else {
            self.stack.push((value, value));
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(value);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        assert_eq!(s.get_min(), -2);
    }

    #[test]
    fn min_reverts_on_pop() {
        let mut s = MinStack::new();
        s.push(5);
        assert_eq!(s.get_min(), 5);
        assert_eq!(s.top(), 5);

        s.push(3);
        assert_eq!(s.get_min(), 3); // new min

        s.push(7);
        assert_eq!(s.get_min(), 3); // 7 doesn't lower the min
        assert_eq!(s.top(), 7);

        s.pop(); // remove 7
        assert_eq!(s.get_min(), 3); // still 3

        s.pop(); // remove 3
        assert_eq!(s.get_min(), 5); // min reverts to 5
        assert_eq!(s.top(), 5);
    }

    #[test]
    fn duplicate_mins() {
        let mut s = MinStack::new();
        s.push(2);
        s.push(2);
        s.push(1);
        assert_eq!(s.get_min(), 1);
        s.pop(); // remove 1
        assert_eq!(s.get_min(), 2);
        s.pop(); // remove one of the 2s
        assert_eq!(s.get_min(), 2); // still 2 — the duplicate is still there
        assert_eq!(s.top(), 2);
    }
}
