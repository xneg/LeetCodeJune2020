mod tree_node_inversion;

use std::rc::Rc;
use std::cell::RefCell;
use tree_node_inversion::invert;
pub use tree_node_inversion::TreeNode as TreeNode;

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      invert(&root);
      root
    }
}