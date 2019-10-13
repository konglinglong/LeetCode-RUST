/// ```rust,ignore
/// 83. 删除排序链表中的重复元素
///
/// 给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。
///
/// 示例 1:
///
/// 输入: 1->1->2
/// 输出: 1->2
///
/// 示例 2:
///
/// 输入: 1->1->2->3->3
/// 输出: 1->2->3
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
/// ```

 #[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
     pub val: i32,
     pub next: Option<Box<ListNode>>
 }

 impl ListNode {
     #[inline]
     fn new(val: i32) -> Self {
         ListNode {
             next: None,
             val
         }
     }
 }

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list = head;
    let mut root = Some(Box::new(ListNode::new(0)));
    let mut cursor = &mut root;
    let mut vec:Vec<i32> = Vec::new();
    loop {
        match list {
            Some(node) => {
                if !vec.contains(&node.val) {
                    vec.push(node.val);
                    cursor.as_mut().unwrap().next = Some(Box::new(ListNode::new(node.val)));
                    cursor = &mut cursor.as_mut().unwrap().next;
                }
                list = node.next;
            }
            None => {
                break;
            }
        }
    }
    root.unwrap().next
}


#[cfg(test)]
#[allow(unused_imports)]
mod test
{
    use super::*;

    #[test]
    fn test_delete_duplicates()
    {
//        assert_eq!(delete_duplicates(3), 3);
    }
}
