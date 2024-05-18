use std::ptr;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn merge_sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref()?.next.is_none() {
        return head;
    }

    let mut slow = head.clone();
    let mut fast = head.clone();

    while let Some(ref next_node) = fast {
        if next_node.next.is_none() || next_node.next.as_ref()?.next.is_none() {
            break;
        }
        slow = slow?.next.clone();
        fast = next_node.next.as_ref()?.next.clone();
    }

    let mut right = merge_sort(slow?.next.take());
    let mut left = merge_sort(head);

    let mut sorted_head = Some(Box::new(ListNode::new(0)));
    let mut current = sorted_head.as_mut();

    while left.is_some() && right.is_some() {
        let (selected_node, remaining) = if left.as_ref()?.val < right.as_ref()?.val {
            (left.take(), left.as_mut()?.next.take())
        } else {
            (right.take(), right.as_mut()?.next.take())
        };

        current.as_mut()?.next = selected_node;
        current = current?.as_mut().unwrap().next.as_mut();
    }

    current.as_mut()?.next = left.or(right);

    sorted_head.unwrap().next
}

fn print_list(head: Option<Box<ListNode>>) {
    let mut current = &head;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = &node.next;
    }
    println!("None");
}

fn main() {
    let mut head = Some(Box::new(ListNode::new(4)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(3)));

    println!("Original list:");
    print_list(head.clone());

    let sorted_head = merge_sort(head);

    println!("Sorted list:");
    print_list(sorted_head);
}
