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

  fn swap(&mut self) {
    match self {
      TreeNode{val: _, left: None, right: None} => (),
      TreeNode{val: _, left, right} => {
        std::mem::swap(left, right);
      }
    }
  }
}

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      Self::invert(&root);
      root
    }

    fn invert(x: &Option<Rc<RefCell<TreeNode>>>) {
      match x {
        Some(x) => {
          x.borrow_mut().swap();
          Self::invert(&x.borrow_mut().left);
          Self::invert(&x.borrow_mut().right);
        },
        None => ()
      }
    }
}