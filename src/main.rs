use std::rc::Rc;
use std::cell::RefCell;
use LeetCodeJune2020::Solution;
use LeetCodeJune2020::TreeNode;

fn main() {
    let node1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let node3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let node6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let node9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    let node2 = Some(Rc::new(RefCell::new(TreeNode{val:2, left: node1, right: node3})));
    let node7 = Some(Rc::new(RefCell::new(TreeNode{val:7, left: node6, right: node9})));
    let tree = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: node2, right: node7})));

    let result = Solution::invert_tree(tree);
    println!("Now {:?} will print!", result);
}
