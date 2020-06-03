mod tree_node_inversion;
mod two_cities;

use std::rc::Rc;
use std::cell::RefCell;
use tree_node_inversion::invert;
use two_cities::two_city_sched_cost;
use tree_node_inversion::TreeNode as TreeNode;


pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      invert(&root);
      root
    }

    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
      two_city_sched_cost(costs)
    }
}