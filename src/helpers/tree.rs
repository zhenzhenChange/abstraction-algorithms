use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};

pub type SubTree<T> = Option<RefTree<T>>;
pub type RefTree<T> = Rc<RefCell<TreeNode<T>>>;

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: SubTree<T>,
    pub right: SubTree<T>,
}

impl<T: Copy + Debug + Clone + Default> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    #[inline]
    pub fn into_ref(self) -> RefTree<T> {
        Rc::new(RefCell::new(self))
    }

    #[inline]
    pub fn from_bfs(mut bfs: Vec<Option<T>>) -> SubTree<T> {
        if bfs.is_empty() {
            return None;
        };

        let root = Self::new(bfs.remove(0).unwrap()).into_ref();

        let mut iter = bfs.into_iter().peekable();
        let mut queue = VecDeque::from([root.clone()]);

        while iter.peek().is_some() {
            let head = queue.pop_front().unwrap();

            if let Some(node) = iter.next() {
                if let Some(node) = node {
                    let node = Self::new(node).into_ref();

                    // Left
                    head.borrow_mut().left = Some(node.clone());
                    queue.push_back(node);
                }
            }

            if let Some(node) = iter.next() {
                if let Some(node) = node {
                    let node = Self::new(node).into_ref();

                    // Right
                    head.borrow_mut().right = Some(node.clone());
                    queue.push_back(node);
                }
            }
        }

        Some(root)
    }
}
