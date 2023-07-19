use std::{cell::RefCell, rc::Rc};

use crate::helpers::tree::TreeNode;

/// 00105
///
/// `T-O()`
///
/// `S-O()`
///
/// https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
#[allow(unused)]
pub fn solution(pre_order: Vec<i32>, in_order: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(0))))
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {}

    #[test]
    fn case2() {}
}
