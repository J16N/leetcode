/*
 * @lc app=leetcode id=572 lang=rust
 *
 * [572] Subtree of Another Tree
 */

struct Solution;
use crate::TreeNode;

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
    pub fn is_subtree(root: OptionalTreeNode, sub_root: OptionalTreeNode) -> bool {
        match (root, sub_root) {
            (Some(node), Some(sub_node)) => {
                let n = node.borrow();
                Self::is_same_tree(Some(Rc::clone(&node)), Some(Rc::clone(&sub_node)))
                    || Self::is_subtree(n.left.clone(), Some(Rc::clone(&sub_node)))
                    || Self::is_subtree(n.right.clone(), Some(Rc::clone(&sub_node)))
            }
            (_, None) => true,
            _ => false,
        }
    }

    fn is_same_tree(root: OptionalTreeNode, sub_root: OptionalTreeNode) -> bool {
        match (root, sub_root) {
            (Some(node), Some(sub_node)) => {
                let node = node.borrow();
                let sub_node = sub_node.borrow();
                node.val == sub_node.val
                    && Self::is_same_tree(node.left.clone(), sub_node.left.clone())
                    && Self::is_same_tree(node.right.clone(), sub_node.right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn find_root(root: OptionalTreeNode, sub_root: Rc<RefCell<TreeNode>>) -> OptionalTreeNode {
        if let Some(node) = root.clone() {
            let node = node.borrow();
            let sub_node = sub_root.borrow();
            if node.val == sub_node.val {
                return root;
            }
            let left = Solution::find_root(node.left.clone(), Rc::clone(&sub_root));
            if left.is_some() {
                return left;
            }
            let right = Solution::find_root(node.right.clone(), Rc::clone(&sub_root));
            if right.is_some() {
                return right;
            }
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
    fn test_1_572() {
        let root = build_tree(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let sub_root = build_tree(vec![Some(4), Some(1), Some(2)]);
        assert!(Solution::is_subtree(root, sub_root));
    }

    #[test]
    fn test_2_572() {
        let root = build_tree(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            Some(0),
        ]);
        let sub_root = build_tree(vec![Some(4), Some(1), Some(2)]);
        assert!(!Solution::is_subtree(root, sub_root));
    }

    #[test]
    fn test_3_572() {
        let root = build_tree(vec![Some(1), Some(1)]);
        let sub_root = build_tree(vec![Some(1)]);
        assert!(Solution::is_subtree(root, sub_root));
    }
}
