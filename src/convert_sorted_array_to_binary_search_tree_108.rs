/*
 * @lc app=leetcode id=108 lang=rust
 *
 * [108] Convert Sorted Array to Binary Search Tree
 */

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

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::make_tree(0, nums.len() as i32 - 1, &nums)
    }

    fn make_tree(start: i32, end: i32, nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }

        let mid = start + (end - start) / 2;
        let mut node = TreeNode::new(nums[mid as usize]);
        node.left = Self::make_tree(start, mid - 1, nums);
        node.right = Self::make_tree(mid + 1, end, nums);
        Some(Rc::new(RefCell::new(node)))
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    fn calc_height(root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) -> (i32, i32) {
        if let Some(node) = root {
            nodes.push(node.borrow().val);
            let node = node.borrow();
            let (_, left_height) = calc_height(node.left.clone(), nodes);
            let (_, right_height) = calc_height(node.right.clone(), nodes);
            return (
                i32::abs(left_height - right_height),
                1 + left_height.max(right_height),
            );
        }
        (0, 0)
    }

    #[test]
    fn test_1_108() {
        let nums = vec![-10, -3, 0, 5, 9];
        let size = nums.len();
        let root = Solution::sorted_array_to_bst(nums);
        let mut nodes = vec![];
        assert!(calc_height(root, &mut nodes).0 <= 1 && nodes.len() == size);
    }

    #[test]
    fn test_2_108() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let size = nums.len();
        let root = Solution::sorted_array_to_bst(nums);
        let mut nodes = vec![];
        assert!(calc_height(root, &mut nodes).0 <= 1 && nodes.len() == size);
    }

    #[test]
    fn test_3_108() {
        let nums = vec![
            -93, -89, -85, -76, -56, -53, -20, -10, 20, 28, 41, 50, 66, 70, 87, 88, 91, 94,
        ];
        let size = nums.len();
        let root = Solution::sorted_array_to_bst(nums);
        let mut nodes = vec![];
        assert!(calc_height(root, &mut nodes).0 <= 1 && nodes.len() == size);
    }
}
