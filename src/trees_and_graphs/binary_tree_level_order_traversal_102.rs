/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
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
use std::collections::VecDeque;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut total = 1;
        let mut result: Vec<Vec<i32>> = vec![];
        let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = [root].into();

        while !q.is_empty() {
            let mut nodes = vec![];
            let mut next_total = 0;
            for _ in 0..total {
                if let Some(Some(node)) = q.pop_front() {
                    let node = node.borrow();
                    nodes.push(node.val);
                    if node.left.is_some() {
                        q.push_back(node.left.clone());
                        next_total += 1;
                    }
                    if node.right.is_some() {
                        q.push_back(node.right.clone());
                        next_total += 1;
                    }
                }
            }
            total = next_total;
            if !nodes.is_empty() {
                result.push(nodes);
            }
        }

        result
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test_1_102() {
        let root = build_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = Solution::level_order(root);
        assert_eq![result, vec![vec![3], vec![9, 20], vec![15, 7]]]
    }

    #[test]
    fn test_2_102() {
        let root = build_tree(vec![Some(1)]);
        let result = Solution::level_order(root);
        assert_eq![result, vec![vec![1]]]
    }

    #[test]
    fn test_3_102() {
        let root = build_tree(vec![]);
        let result = Solution::level_order(root);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected)
    }

    #[test]
    fn test_4_102() {
        let root = build_tree(vec![
            Some(1),
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
        ]);
        let result = Solution::level_order(root);
        assert_eq![result, vec![vec![1], vec![2], vec![3], vec![4], vec![5]]]
    }

    #[test]
    fn test_5_102() {
        let root = build_tree(vec![Some(1), Some(2), Some(3), Some(4), None, Some(5)]);
        let result = Solution::level_order(root);
        assert_eq![result, vec![vec![1], vec![2, 3], vec![4, 5]]]
    }
}
