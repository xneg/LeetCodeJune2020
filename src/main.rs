use std::rc::Rc;
use std::cell::RefCell;
use LeetCodeJune2020::Solution;
use LeetCodeJune2020::TreeNode;

fn main() {
    
    let tree = TreeNode{val: 5, left: None, right: None};

    let result = Solution::invert_tree(Some(Rc::new(RefCell::new(tree))));
    println!("Now {:?} will print!", result);
}
