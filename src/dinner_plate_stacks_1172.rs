/*
 * @lc app=leetcode id=1172 lang=rust
 *
 * [1172] Dinner Plate Stacks
 */

// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
struct DinnerPlates {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    available: BinaryHeap<Reverse<usize>>,
}

#[allow(dead_code)]
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            stacks: Vec::new(),
            available: BinaryHeap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        while let Some(Reverse(i)) = self.available.pop() {
            if let Some(stack) = self.stacks.get_mut(i) {
                if stack.len() < self.capacity {
                    stack.push(val);
                    return;
                }
            }
        }

        if let Some(stack) = self.stacks.last_mut() {
            if stack.len() < self.capacity {
                stack.push(val);
                return;
            }
        }

        self.stacks.push(vec![val]);
    }

    fn pop(&mut self) -> i32 {
        let Some(last_stack) = self.stacks.last_mut() else {
            return -1;
        };

        if let Some(n) = last_stack.pop() {
            if last_stack.is_empty() {
                self.stacks.pop();
            }
            n
        } else {
            self.stacks.pop();
            self.pop()
        }
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let i = index as usize;
        if let Some(stack) = self.stacks.get_mut(i) {
            if let Some(n) = stack.pop() {
                self.available.push(Reverse(i));
                return n;
            }
        }
        -1
    }
}

// // Your DinnerPlates object will be instantiated and called as such:
// let obj = DinnerPlates::new(capacity);
// obj.push(val);
// let ret_2: i32 = obj.pop();
// let ret_3: i32 = obj.pop_at_stack(index);
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dinner_plate_stacks_testcase_1() {
        let mut d = DinnerPlates::new(2);
        d.push(1);
        d.push(2);
        d.push(3);
        d.push(4);
        d.push(5);
        assert_eq!(d.pop_at_stack(0), 2);
        d.push(20);
        d.push(21);
        assert_eq!(d.pop_at_stack(0), 20);
        assert_eq!(d.pop_at_stack(2), 21);
        assert_eq!(d.pop(), 5);
        assert_eq!(d.pop(), 4);
        assert_eq!(d.pop(), 3);
        assert_eq!(d.pop(), 1);
        assert_eq!(d.pop(), -1);
    }

    #[test]
    fn dinner_plate_stacks_testcase_2() {
        let mut d = DinnerPlates::new(2);
        d.push(1);
        d.push(2);
        d.push(3);
        d.push(4);
        assert_eq!(d.pop(), 4);
        d.push(4);
        assert_eq!(d.pop_at_stack(1), 4);
    }
}
