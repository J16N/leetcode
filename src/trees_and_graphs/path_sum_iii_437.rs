/*
 * @lc app=leetcode id=437 lang=rust
 *
 * [437] Path Sum III
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
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut path_count = HashMap::new();
        Self::count_path_sum(root, target_sum, 0i128, &mut path_count)
    }

    fn count_path_sum(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        mut running_sum: i128,
        path_count: &mut HashMap<i128, i32>,
    ) -> i32 {
        match root {
            Some(node) => {
                let mut node = node.borrow_mut();
                running_sum += node.val as i128;
                let sum = running_sum - target_sum as i128;
                let mut total_paths = *path_count.get(&sum).unwrap_or(&0);

                if running_sum == target_sum as i128 {
                    total_paths += 1;
                }

                *path_count.entry(running_sum).or_insert(0) += 1;
                total_paths +=
                    Self::count_path_sum(node.left.take(), target_sum, running_sum, path_count);
                total_paths +=
                    Self::count_path_sum(node.right.take(), target_sum, running_sum, path_count);
                *path_count.get_mut(&running_sum).unwrap() -= 1;
                total_paths
            }
            None => 0,
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test_1_437() {
        let input = vec![
            Some(10),
            Some(5),
            Some(-3),
            Some(3),
            Some(2),
            None,
            Some(11),
            Some(3),
            Some(-2),
            None,
            Some(1),
        ];
        let root = build_tree(input);
        let target_sum = 8;
        let result = 3;
        assert_eq!(Solution::path_sum(root, target_sum), result);
    }

    #[test]
    fn test_2_437() {
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
        let result = 3;
        assert_eq!(Solution::path_sum(root, target_sum), result);
    }

    #[test]
    fn test_3_437() {
        let input = vec![
            Some(1000000000),
            Some(1000000000),
            None,
            Some(294967296),
            None,
            Some(1000000000),
            None,
            Some(1000000000),
            None,
            Some(1000000000),
        ];
        let root = build_tree(input);
        let target_sum = 8;
        let result = 0;
        assert_eq!(Solution::path_sum(root, target_sum), result);
    }

    #[test]
    fn test_4_437() {
        let input = vec![
            Some(1),
            Some(-2),
            Some(-3),
            Some(1),
            Some(3),
            Some(-2),
            None,
            Some(-1),
        ];
        let root = build_tree(input);
        let target_sum = -1;
        let result = 4;
        assert_eq!(Solution::path_sum(root, target_sum), result);
    }
}
