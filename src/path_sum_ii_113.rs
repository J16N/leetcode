/*
 * @lc app=leetcode id=113 lang=rust
 *
 * [113] Path Sum II
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        fn path_sum(
            root: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i32,
            temp: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if let Some(node) = root {
                let mut node = node.borrow_mut();
                let target_sum = target_sum - node.val;

                temp.push(node.val);
                if node.left.is_none() && node.right.is_none() {
                    if target_sum == 0 {
                        println!("{:?}", temp);
                        result.push(temp.clone());
                    }
                    temp.pop();
                    return;
                }

                path_sum(node.left.take(), target_sum, temp, result);
                path_sum(node.right.take(), target_sum, temp, result);
                temp.pop();
            }
        }
        let mut temp = vec![];
        path_sum(root, target_sum, &mut temp, &mut result);
        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test_1_113() {
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
            Some(5),
            Some(1),
        ];
        let root = build_tree(input);
        let target_sum = 22;
        let result = Solution::path_sum(root, target_sum);
        let expected = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2_113() {
        let input = vec![Some(1), Some(2), Some(3)];
        let root = build_tree(input);
        let target_sum = 5;
        let result = Solution::path_sum(root, target_sum);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }
}
