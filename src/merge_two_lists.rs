/// ```rust,ignore
/// 将两个有序链表合并为一个新的有序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
///
/// 示例：
///
/// 输入：1->2->4, 1->3->4
/// 输出：1->1->2->3->4->4
///
/// 来源：力扣（LeetCode）
/// 链接：https:/// leetcode-cn.com/problems/merge-two-sorted-lists
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
/// ```

// Definition for singly-linked list.
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

#[allow(dead_code, non_snake_case)]
pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let (mut l1Cursor, mut l2Cursor) = (&mut l1, &mut l2);
    let mut root = Some(Box::new(ListNode::new(0)));
    let mut cursor = &mut root;

    let (mut l1IsEnd, mut l2IsEnd);
    loop {
        if *l1Cursor == None && *l2Cursor == None{
            break;
        }

        l1IsEnd = false;
        l2IsEnd = false;

        let l1Val = match l1Cursor {
            Some(node) => {
                node.val
            }
            None => {
                l1IsEnd = true;
                0
            }
        };
        let l2Val = match l2Cursor {
            Some(node) => {
                node.val
            }
            None => {
                l2IsEnd = true;
                0
            }
        };

        let mut nextIsL1 = false;
        if l1IsEnd || l2IsEnd {
            if l2IsEnd {
                nextIsL1 = true;
            }
        } else {
            if l1Val < l2Val {
                nextIsL1 = true;
            }
        }
        if nextIsL1 {
            cursor.as_mut().unwrap().next = Some(Box::new(ListNode::new(l1Val)));
            cursor = &mut cursor.as_mut().unwrap().next;
            l1Cursor = &mut (*l1Cursor).as_mut().unwrap().next;
            continue;
        } else {
            cursor.as_mut().unwrap().next = Some(Box::new(ListNode::new(l2Val)));
            cursor = &mut cursor.as_mut().unwrap().next;
            l2Cursor = &mut (*l2Cursor).as_mut().unwrap().next;
        }
    }
    root.unwrap().next
}

#[cfg(test)]
mod test {
    use super::merge_two_lists;

    #[test]
    fn test_merge_two_lists() {
    }
}