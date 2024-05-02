/*
 * @lc app=leetcode id=236 lang=rust
 *
 * [236] Lowest Common Ancestor of a Binary Tree
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

type OptionalTreeNode = Option<Rc<RefCell<TreeNode>>>;

#[allow(dead_code)]
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        Solution::dfs(root, p, q)
    }

    fn dfs(root: OptionalTreeNode, p: i32, q: i32) -> OptionalTreeNode {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            if node.val == p || node.val == q {
                return root;
            }
            let left = Solution::dfs(node.left.take(), p, q);
            let right = Solution::dfs(node.right.take(), p, q);

            return match (&left, &right) {
                (Some(_), Some(_)) => root,
                (Some(_), None) => left,
                (None, Some(_)) => right,
                _ => None,
            };
        }
        root
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test_1_236() {
        let root = build_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = build_tree(vec![Some(5)]);
        let q = build_tree(vec![Some(1)]);
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_2_236() {
        let root = build_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = build_tree(vec![Some(5)]);
        let q = build_tree(vec![Some(4)]);
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 5);
    }
}
