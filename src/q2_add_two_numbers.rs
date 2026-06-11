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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut cur = &mut dummy;
    let (mut p1, mut p2) = (l1, l2);
    let mut carry = 0;
    while p1.is_some() || p2.is_some() || carry != 0 {
        let mut sum = carry;
        p1 = p1.and_then(|node| {
            sum += node.val;
            node.next
        });
        p2 = p2.and_then(|node| {
            sum += node.val;
            node.next
        });
        carry = sum / 10;
        let new_value = sum % 10;
        cur.next = Some(Box::new(ListNode::new(new_value)));
        cur = cur.next.as_mut().unwrap();
    }
    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &x in v.iter().rev() {
            let mut node = Box::new(ListNode::new(x));
            node.next = head;
            head = Some(node);
        }
        head
    }

    fn to_vec(mut l: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        while let Some(n) = l {
            out.push(n.val);
            l = n.next;
        }
        out
    }

    #[test]
    fn example_342_plus_465() {
        assert_eq!(to_vec(add_two_numbers(from_vec(vec![2, 4, 3]), from_vec(vec![5, 6, 4]))), vec![7, 0, 8]);
    }

    #[test]
    fn example_zeros() {
        assert_eq!(to_vec(add_two_numbers(from_vec(vec![0]), from_vec(vec![0]))), vec![0]);
    }

    #[test]
    fn example_carry_chain() {
        assert_eq!(
            to_vec(add_two_numbers(
                from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
                from_vec(vec![9, 9, 9, 9])
            )),
            vec![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }
}