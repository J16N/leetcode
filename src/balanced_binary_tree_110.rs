/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root).is_some()
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            let left = Self::dfs(node.borrow().left.clone());
            let right = Self::dfs(node.borrow().right.clone());
            if let Some(l) = left {
                if let Some(r) = right {
                    if (l - r).abs() <= 1 {
                        return Some(1 + l.max(r));
                    }
                }
            }
            return None;
        }
        Some(0)
    }
}
// @lc code=end
#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test_1() {
        let input = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = build_tree(input);
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn test_2() {
        let input = vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ];
        let root = build_tree(input);
        assert!(!Solution::is_balanced(root));
    }
}
