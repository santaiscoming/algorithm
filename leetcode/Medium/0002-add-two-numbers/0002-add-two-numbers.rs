// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>, 
        mut l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut curr = &mut head;
        let mut w = 0;

        while l1.is_some() || l2.is_some() || w > 0 {
            let mut sum = w;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            w = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
        }

        head.next
    }
}