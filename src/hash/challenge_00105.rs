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
    use super::solution;

    use crate::helpers::tree::TreeNode;

    #[test]
    fn case1() {
        let preset_arg0 = vec![3, 9, 20, 15, 7];
        let preset_arg1 = vec![9, 3, 15, 20, 7];

        let preset_ret = TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);

        assert_eq!(preset_ret, solution(preset_arg0, preset_arg1));
    }

    #[test]
    fn case2() {
        let preset_arg0 = vec![-1];
        let preset_arg1 = vec![-1];

        let preset_ret = TreeNode::from(vec![Some(-1)]);

        assert_eq!(preset_ret, solution(preset_arg0, preset_arg1));
    }
}
