use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(x) => invert_t(x)
        }
    }

    fn invert_t(root: Rc<RefCell<TreeNode>>) -> Rc<RefCel<TreeNode>> {
        match &*root.borrow() {
            TreeNode {val, left: None, right: None} => root,
            TreeNode {val, left, right} => TreeNode {val: val, left: invert_tree(right), right: invert_tree(left)}
        }

    }
}