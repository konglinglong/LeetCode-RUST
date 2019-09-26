/// ```rust,ignore
/// 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
///
/// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
///
/// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
///
/// 示例：
///
/// 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
/// 输出：7 -> 0 -> 8
/// 原因：342 + 465 = 807
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

#[allow(dead_code, non_snake_case)]
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut root = Some(Box::new(ListNode::new(0)));
    let mut cursor = &mut root;
    let mut carry = 0;
    loop {
        if l1 == None && l2 == None && carry == 0 {
            break;
        }
        let l1Val = match l1 {
            Some(node) => {
                l1 = node.next;
                node.val
            }
            None => {
                0
            }
        };
        let l2Val = match l2 {
            Some(node) => {
                l2 = node.next;
                node.val
            }
            None => {
                0
            }
        };

        let sum = l1Val + l2Val + carry;
        carry = sum / 10;
        cursor.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
        cursor = &mut cursor.as_mut().unwrap().next;
    }
    root.unwrap().next
}

#[cfg(test)]
mod test {
    use super::add_two_numbers;

    #[test]
    fn test_add_two_numbers() {

    }
}
