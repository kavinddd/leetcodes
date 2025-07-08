use std::{cell::RefCell, collections::VecDeque, rc::Rc};

/**
 * Definition for a binary tree node.
* */

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {}

fn maximum_depth_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> u32 {
    if root.is_none() {
        return 0;
    }

    let node = root.unwrap();
    let node_ref = node.borrow();

    let left = maximum_depth_of_binary_tree(node_ref.left.clone());
    let right = maximum_depth_of_binary_tree(node_ref.right.clone());

    1 + left.max(right)
}
