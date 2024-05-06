/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let mut node = node.borrow_mut();
                let target_sum = target_sum - node.val;
                if node.left.is_none() && node.right.is_none() {
                    return target_sum == 0;
                }
                Solution::has_path_sum(node.left.take(), target_sum)
                    || Solution::has_path_sum(node.right.take(), target_sum)
            }
            None => false,
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test_1_112() {
        let input = vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ];
        let root = build_tree(input);
        assert!(Solution::has_path_sum(root, 22));
    }

    #[test]
    fn test_2_112() {
        let input = vec![Some(1), Some(2), Some(3)];
        let root = build_tree(input);
        assert!(!Solution::has_path_sum(root, 5));
    }

    #[test]
    fn test_3_112() {
        let input = vec![];
        let root = build_tree(input);
        assert!(!Solution::has_path_sum(root, 0));
    }

    #[test]
    fn test_4_112() {
        let input = vec![Some(1), Some(2)];
        let root = build_tree(input);
        assert!(!Solution::has_path_sum(root, 1));
    }
}
