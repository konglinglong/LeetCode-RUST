/// ```rust,ignore
/// 104. 二叉树的最大深度
///
/// 给定一个二叉树，找出其最大深度。
///
/// 二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。
///
/// 说明: 叶子节点是指没有子节点的节点。
///
/// 示例：
/// 给定二叉树 [3,9,20,null,null,15,7]，
///
/// 3
/// \
/// 9  20
///  \
/// 15   7
///
/// 返回它的最大深度 3 。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/maximum-depth-of-binary-tree
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
/// ```

// Definition for a binary tree node.
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

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut max_depth = 0;
    let mut queue = VecDeque::new();
    queue.push_back((1, root.clone()));
    while !queue.is_empty() {
        if let Some((depth, Some(node))) = queue.pop_front() {
            if depth > max_depth {
                max_depth = depth;
            }
            queue.push_back((depth + 1, node.borrow().left.clone()));
            queue.push_back((depth + 1, node.borrow().right.clone()));
        }
    }
    max_depth
}

#[cfg(test)]
#[allow(unused_imports)]
mod test
{
    use super::*;

    #[test]
    fn test_max_depth()
    {
//        assert_eq!(max_depth("2".to_string(), "3".to_string()), "6".to_string());
    }
}
