/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
 */
use crate::TreeNode;

struct Solution;

// @lc code=start
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

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid(root, i64::MIN, i64::MAX)
    }

    fn is_valid(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match root {
            Some(node) => {
                let mut node = node.borrow_mut();
                (node.val as i64) > min
                    && (node.val as i64) < max
                    && Self::is_valid(node.left.take(), min, node.val as i64)
                    && Self::is_valid(node.right.take(), node.val as i64, max)
            }
            None => true,
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test_1_98() {
        let input = vec![Some(2), Some(1), Some(3)];
        let root = build_tree(input);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn test_2_98() {
        let input = vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)];
        let root = build_tree(input);
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn test_3_98() {
        let input = vec![Some(1), Some(1)];
        let root = build_tree(input);
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn test_4_98() {
        let input = vec![Some(5), Some(4), Some(6), None, None, Some(3), Some(7)];
        let root = build_tree(input);
        assert!(!Solution::is_valid_bst(root));
    }
}
