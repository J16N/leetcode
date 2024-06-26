mod stack_and_queue;
mod trees_and_graphs;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type OptionalTreeNode = Option<Rc<RefCell<TreeNode>>>;
pub struct TreeNode {
    pub val: i32,
    pub left: OptionalTreeNode,
    pub right: OptionalTreeNode,
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

pub fn build_tree(input: Vec<Option<i32>>) -> OptionalTreeNode {
    if input.is_empty() || input[0].is_none() {
        return None;
    }

    let mut q: VecDeque<Option<i32>> = input.into_iter().collect();
    let root = Rc::new(RefCell::new(TreeNode::new(q.pop_front().unwrap().unwrap())));
    let mut nodes: VecDeque<Rc<RefCell<TreeNode>>> = [root.clone()].into();
    let mut total = 1;

    while !q.is_empty() {
        let mut next_total = 0;
        for _ in 0..total {
            let curr = nodes.pop_front().unwrap();
            if let Some(Some(n)) = q.pop_front() {
                let node = Rc::new(RefCell::new(TreeNode::new(n)));
                nodes.push_back(node.clone());
                curr.borrow_mut().left = Some(node.clone());
                next_total += 1;
            }
            if let Some(Some(n)) = q.pop_front() {
                let node = Rc::new(RefCell::new(TreeNode::new(n)));
                nodes.push_back(node.clone());
                curr.borrow_mut().right = Some(node.clone());
                next_total += 1;
            }
        }
        total = next_total;
    }

    Some(root)
}
