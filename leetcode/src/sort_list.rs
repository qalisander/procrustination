use std::iter;

// https://leetcode.com/problems/sort-list/description/
#[derive(Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl PartialEq for ListNode {
    fn eq(&self, other: &Self) -> bool {
        let mut left = self;
        let mut right = other;
        loop {
            if left.val != right.val {
                return false;
            }
            match (&left.next, &right.next) {
                (Some(left_next), Some(right_next)) => {
                    left = &**left_next;
                    right = &**right_next;
                }
                (None, Some(_)) | (Some(_), None) => return false,
                (None, None) => return true,
            }
        }
    }
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_array<const T: usize>(arr: [i32; T]) -> Option<Box<ListNode>> {
        arr.into_iter().rev().fold(None, |acc, num| {
            Some(Box::new(if acc.is_some() {
                ListNode {
                    val: num,
                    next: acc,
                }
            } else {
                ListNode::new(num)
            }))
        })
    }
}

struct Solution;

// Basically implementation of the merge sort for linked list
// Complexity:
// time - O(n*log(n))
// memory - O(1)

type Link = Option<Box<ListNode>>;
impl Solution {
    pub fn sort_list(head: Link) -> Link {
        let len = Self::len(&head);
        match len {
            0 => None,
            1 => head,
            _ => {
                let split_index = len / 2;
                let (left, right) = Self::split(head, split_index).expect("list should be split");
                let sorted_left = Self::sort_list(left);
                let sorted_right = Self::sort_list(right);
                Self::merge(sorted_left, sorted_right)
            }
        }
    }

    // Return none if index is out of bounds of linked list
    fn split(mut head: Link, mut split_index: usize) -> Option<(Link, Link)> {
        let mut end_of_first = &mut head;
        while split_index > 1 {
            end_of_first = &mut end_of_first.as_mut()?.next;
            split_index -= 1;
        }
        let second = end_of_first.as_mut()?.next.take();
        Some((head, second))
    }

    fn len(head: &Link) -> usize {
        let mut node = head;
        let mut len = 0;
        while let Some(next) = node {
            len += 1;
            node = &next.next
        }
        len
    }

    pub fn merge(mut left: Link, mut right: Link) -> Link {
        // Create head node for convenience
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        loop {
            let tail_next = match (left.take(), right.take()) {
                (Some(mut l), Some(mut r)) => {
                    if l.val <= r.val {
                        left = l.next.take();
                        right = Some(r);
                        l
                    } else {
                        right = r.next.take();
                        left = Some(l);
                        r
                    }
                }
                (Some(mut l), None) => {
                    left = l.next.take();
                    l
                }
                (None, Some(mut r)) => {
                    right = r.next.take();
                    r
                }
                (None, None) => return head.next,
            };
            tail.next = Some(tail_next);
            tail = tail.next.as_mut().unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sort_list::{ListNode, Solution};

    #[test]
    fn check0() {
        let head = ListNode::from_array([-1, 5, 3, 4, 0]);
        let expected = ListNode::from_array([-1, 0, 3, 4, 5]);

        let ans = Solution::sort_list(head);
        assert_eq!(expected, ans);
    }

    #[test]
    fn check_merge() {
        let left = ListNode::from_array([1, 3, 4, 6]);
        let right = ListNode::from_array([2, 3, 5]);
        let expected = ListNode::from_array([1, 2, 3, 3, 4, 5, 6]);
        let merged = Solution::merge(left, right);
        assert_eq!(expected, merged);
    }
}
