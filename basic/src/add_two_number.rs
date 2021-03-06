use eager_log::*;
use std::cmp::Ordering;

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
    let mut carry = 0;
    let mut result: Option<Box<ListNode>> = None; //: Option<Box<ListNode>>
    let mut current = &mut result; // 获取一个可变引用，表示可以改变result的值。current也需要可变，用于修改指向

    let mut f1 = &l1;
    let mut f2 = &l2;
    while !f1.is_none() || !f2.is_none() || carry == 1 {
        let v1 = match f1.as_ref() {
            Some(x) => x.val,
            None => 0,
        };
        let v2 = match f2.as_ref() {
            Some(x) => x.val,
            None => 0,
        };
        let val = match (v1 + v2 + carry).cmp(&10) {
            Ordering::Greater | Ordering::Equal => {
                let n = (v1 + v2 + carry) % 10;
                carry = 1;
                n
            }
            _ => {
                let n = v1 + v2 + carry;
                carry = 0;
                n
            }
        };
        *current = Some(Box::new(ListNode::new(val)));
        info!("carry:{}, value:{}", carry, val);
        current = &mut current.as_mut().unwrap().next;

        f1 = match f1 {
            Some(x) => &(x.next),
            None => &None,
        };
        f2 = match f2 {
            Some(x) => &(x.next),
            None => &None,
        };
    }
    return result;
}

pub fn add_two_numbers2(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut curr = &mut dummy;
    let mut t = (l1, l2, 0, 0); // l1,l2,sum,offset

    loop {
        t = match t {
            (None, None, _, 0) => break,
            (None, None, _, offset) => (None, None, offset, 0),
            (Some(list), None, _, offset) | (None, Some(list), _, offset) => (
                list.next,
                None,
                (list.val + offset) % 10,
                (list.val + offset) / 10,
            ),
            (Some(l1), Some(l2), _, offset) => (
                l1.next,
                l2.next,
                (l1.val + l2.val + offset) % 10,
                (l1.val + l2.val + offset) / 10,
            ),
        };
        *curr = Some(Box::new(ListNode::new(t.2)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    dummy
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

pub fn carried(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: carried(
                l1.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                l2.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                carry / 10,
            ),
            val: carry % 10,
        }))
    }
}

#[test]
fn test_add_two_numbers() {
    let l0 = ListNode::new(1);
    let mut l1 = ListNode::new(2);
    let mut l2 = ListNode::new(5);
    l1.next = Some(Box::new(l0.clone()));
    l2.next = Some(Box::new(l1));

    trace!("{:?}", l2);

    let list = Some(Box::new(l2));
    debug_list(&list);
    let result = add_two_numbers(list.clone(), Some(Box::new(l0)));
    // debug_list(&result);
    debug!("{:?}", result);
}
