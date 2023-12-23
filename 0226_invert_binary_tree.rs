use std::rc::Rc;
use std::cell::RefCell;

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

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|root| {
            match root.borrow_mut() {
                mut node => {
                    let left = node.left.clone();
                    let right = node.right.clone();
                    node.left = Self::invert_tree(right);
                    node.right = Self::invert_tree(left);
                }
            };
            root
        })
    }
}

struct Solution;

fn main () {}
