use eager_log::*;
use std::cmp::Ordering;
use std::fs::read;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut first = match l1 {
        Some(x) => x,
        None => return l2,
    };
    let mut second = match l2 {
        Some(x) => x,
        None => return Some(first),
    };

    let mut carry = 0;
    let mut ret = Some(Box::new(ListNode::new(first.val)));

    loop {
        trace!("{:?}", &ret);
        let val = match (first.val + second.val + carry).cmp(&10) {
            Ordering::Greater | Ordering::Equal => {
                carry = 1;
                (first.val + second.val + carry) % 10
            }
            _ => {
                carry = 0;
                first.val + second.val + carry
            }
        };

        let next = Some(Box::new(ListNode::new(val)));
        match ret {
            Some(ref mut x) => x.next = next,
            _ => (),
        }

        first = match first.next {
            Some(x) => x,
            None => break,
        };
        second = match second.next {
            Some(x) => x,
            None => break,
        };
    }

    return ret;
}

fn debug_list(l: &Option<Box<ListNode>>) {
    let mut list = match l {
        Some(x) => x,
        None => return,
    };
    loop {
        debug!("{}", list.val);
        list = match &list.next {
            Some(x) => x,
            None => return,
        };
    }
}

#[test]
fn test_add_two_numbers() {
    let l0 = ListNode::new(1);
    let mut l1 = ListNode::new(2);
    let mut l2 = ListNode::new(3);
    l1.next = Some(Box::new(l0));
    l2.next = Some(Box::new(l1));

    trace!("{:?}", l2);

    let list = Some(Box::new(l2));
    add_two_numbers(list.clone(), list);
}
