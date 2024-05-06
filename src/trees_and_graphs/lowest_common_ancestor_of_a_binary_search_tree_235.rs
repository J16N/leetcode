/*
 * @lc app=leetcode id=235 lang=rust
 *
 * [235] Lowest Common Ancestor of a Binary Search Tree
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
        root: OptionalTreeNode,
        p: OptionalTreeNode,
        q: OptionalTreeNode,
    ) -> OptionalTreeNode {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        Solution::dfs(root, p, q)
    }

    fn dfs(root: OptionalTreeNode, p: i32, q: i32) -> OptionalTreeNode {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            if p < node.val && q < node.val {
                return Solution::dfs(node.left.take(), p, q);
            }

            if p > node.val && q > node.val {
                return Solution::dfs(node.right.take(), p, q);
            }

            return root;
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
    fn test_1_235() {
        let root = build_tree(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = build_tree(vec![Some(2)]);
        let q = build_tree(vec![Some(8)]);
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 6);
    }

    #[test]
    fn test_2_235() {
        let root = build_tree(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = build_tree(vec![Some(2)]);
        let q = build_tree(vec![Some(4)]);
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }
}
