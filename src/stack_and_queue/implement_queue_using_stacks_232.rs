/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
#[allow(dead_code)]
struct MyQueue {
    stack: Vec<i32>,
    temp: Vec<i32>,
}

#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            temp: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        while let Some(el) = self.stack.pop() {
            self.temp.push(el);
        }
        self.stack.push(x);
        while let Some(el) = self.temp.pop() {
            self.stack.push(el);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_232() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert!(!queue.empty());
    }
}
