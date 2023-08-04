use crate::helpers::tree::{SubTree, TreeNode};

/// 00105
///
/// `T-O(N)`
///
/// `S-O(N)`
///
/// https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
#[allow(unused)]
pub fn solution(mut pre_order: Vec<i32>, mut in_order: Vec<i32>) -> SubTree<i32> {
    if pre_order.is_empty() || in_order.is_empty() {
        return None;
    }

    let root = pre_order.remove(0);
    let root_idx = in_order.iter().position(|&node| node == root).unwrap();
    let root_node = TreeNode::new(root).into_ref();

    let (in_r, in_l) = (in_order.split_off(root_idx + 1), in_order);
    let (pre_r, pre_l) = (pre_order.split_off(root_idx), pre_order);

    root_node.borrow_mut().left = solution(pre_l, in_l);
    root_node.borrow_mut().right = solution(pre_r, in_r);

    Some(root_node)
}

#[cfg(test)]
mod tests {
    use super::solution;

    use crate::helpers::tree::TreeNode;

    #[test]
    fn case1() {
        let preset_arg0 = vec![3, 9, 20, 15, 7];
        let preset_arg1 = vec![9, 3, 15, 20, 7];

        let preset_ret = TreeNode::from_bfs(vec![
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

        let preset_ret = TreeNode::from_bfs(vec![Some(-1)]);

        assert_eq!(preset_ret, solution(preset_arg0, preset_arg1));
    }
}
