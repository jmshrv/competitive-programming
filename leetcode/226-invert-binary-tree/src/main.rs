use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }

        let binding = root.clone().unwrap();
        let mut node_ref = binding.borrow_mut();

        (node_ref.left, node_ref.right) = (node_ref.right.clone(), node_ref.left.clone());

        Solution::invert_tree(node_ref.left.clone());
        Solution::invert_tree(node_ref.right.clone());

        root
    }
}

fn main() {
    // not typing all of that
}
