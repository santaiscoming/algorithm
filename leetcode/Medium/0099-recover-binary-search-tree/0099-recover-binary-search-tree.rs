// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut first: Option<Rc<RefCell<TreeNode>>> = None;
        let mut second: Option<Rc<RefCell<TreeNode>>> = None;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        fn inorder(
            node: &Option<Rc<RefCell<TreeNode>>>,
            first: &mut Option<Rc<RefCell<TreeNode>>>,
            second: &mut Option<Rc<RefCell<TreeNode>>>,
            prev: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(n) = node {
                let node_ref = n.borrow();

                inorder(&node_ref.left, first, second, prev);

                if let Some(p) = prev {
                    if p.borrow().val > node_ref.val {
                        if first.is_none() {
                            *first = Some(Rc::clone(p));
                        }
                        *second = Some(Rc::clone(n));
                    }
                }
                *prev = Some(Rc::clone(n));

                inorder(&node_ref.right, first, second, prev);
            }
        }

        inorder(root, &mut first, &mut second, &mut prev);

        if let (Some(f), Some(s)) = (first, second) {
            let temp = f.borrow().val;
            f.borrow_mut().val = s.borrow().val;
            s.borrow_mut().val = temp;
        }
    }
}