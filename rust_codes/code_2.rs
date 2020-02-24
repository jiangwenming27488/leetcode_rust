#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut lhs, mut rhs) = (l1, l2);
        let mut res: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let (mut tmp, mut sum) = (&mut res, 0);
        let (mut lhs_tag, mut rhs_tag) = (lhs.is_some(), rhs.is_some());
        while lhs_tag || rhs_tag || sum > 0 {
            if lhs_tag {
                sum += lhs.as_ref().unwrap().val;
                lhs = lhs.unwrap().next;
                lhs_tag = lhs.is_some();
            }
            if rhs_tag {
                sum += rhs.as_ref().unwrap().val;
                rhs = rhs.unwrap().next;
                rhs_tag = rhs.is_some();
            }
            tmp.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            tmp = &mut tmp.as_mut().unwrap().next;
            sum /= 10;
        }
        res.unwrap().next
    }
}

fn main() {
    let mut first = Some(Box::new(ListNode::new(3)));
    first.as_mut().unwrap().next = None;
    let mut second = Some(Box::new(ListNode::new(4)));
    second.as_mut().unwrap().next = first;
    let mut left = Some(Box::new(ListNode::new(2)));
    left.as_mut().unwrap().next = second;
    println!("left:{:?}", left);

    let mut first = Some(Box::new(ListNode::new(4)));
    first.as_mut().unwrap().next = None;
    let mut second = Some(Box::new(ListNode::new(6)));
    second.as_mut().unwrap().next = first;
    let mut right = Some(Box::new(ListNode::new(5)));
    right.as_mut().unwrap().next = second;
    println!("right:{:?}", right);

    let mut first = Some(Box::new(ListNode::new(8)));
    first.as_mut().unwrap().next = None;
    let mut second = Some(Box::new(ListNode::new(0)));
    second.as_mut().unwrap().next = first;
    let mut res = Some(Box::new(ListNode::new(7)));
    res.as_mut().unwrap().next = second;
    println!("res{:?}", res);

    assert_eq!(res, Solution::add_two_numbers(left, right));
}


