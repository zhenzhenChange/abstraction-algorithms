use std::{cell::RefCell, rc::Rc};

type SubTree<T> = Option<Rc<RefCell<TreeNode<T>>>>;

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: SubTree<T>,
    pub right: SubTree<T>,
}
