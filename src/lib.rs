use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub struct SimpleNode {
  pub val: i32,
  pub left: Rc<RefCell<SimpleNode>>,
  pub right: Rc<RefCell<SimpleNode>>
}

impl SimpleNode {
  pub fn swap(this: &mut SimpleNode) {
    let l = Rc::clone(&this.left);
    let r = Rc::clone(&this.right);
    l.swap(&r);
  }
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

  pub fn swap(&mut self) {
    match self {
        TreeNode{val: _, left: Some(l), right: Some(r)} => {
          let l = Rc::clone(&l);
          let r = Rc::clone(&r);
          l.swap(&r);
        },
        TreeNode{val: _, left: Some(l), right: None} => {
          let x = Rc::clone(&l);
          &self.right.replace(x);
          self.left = None;
        },
        TreeNode{val: _, left: None, right: Some(r)} => {
          let x = Rc::clone(&r);
          &self.left.replace(x);
          self.right = None;
        },
        _ => ()
    }
  }

  pub fn swap_total(&mut self) {
    match self {
        TreeNode{val: _, left: Some(l), right: Some(r)} => {
          let l = Rc::clone(l);
          let r = Rc::clone(r);
          (*l.borrow_mut()).swap_total();
          (*r.borrow_mut()).swap_total();
          l.swap(&r);
        },
        TreeNode{val: _, left: Some(l), right: None} => {
          let x = Rc::clone(l);
          (*x.borrow_mut()).swap_total();
          &self.right.replace(x);
          self.left = None;
        },
        TreeNode{val: _, left: None, right: Some(r)} => {
          let x = Rc::clone(r);
          (*x.borrow_mut()).swap_total();
          &self.left.replace(x);
          self.right = None;
        },
        _ => ()
    }
  }
}

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      root
        // match root {
        //     None => None,
        //     Some(x) => Some(Self::invert_t(x))
        // }
    }
}