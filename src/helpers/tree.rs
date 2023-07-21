use std::{cell::RefCell, rc::Rc};

type SubTree<T> = Option<Rc<RefCell<TreeNode<T>>>>;

impl<T: Clone + Default> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    #[inline]
    pub fn from(v: Vec<Option<T>>) -> SubTree<T> {
        if v.is_empty() {
            return None;
        }

        let node = Self::new(v[0].clone().unwrap());
        let root = Some(Rc::new(RefCell::new(node)));

        root
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: SubTree<T>,
    pub right: SubTree<T>,
}
