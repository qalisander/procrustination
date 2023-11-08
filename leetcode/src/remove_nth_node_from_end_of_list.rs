// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

// Definition for singly-linked list.
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


struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n <= 0 { panic!("n <= 0"); }

        let mut fast = &mut head as *mut Option<Box<ListNode>>;
        let mut slow = fast;
        for _ in 0..n {
            if fast.is_null() {
                return head;
            }
            fast = advance_node(fast);
        }

        return loop {
            fast = advance_node(fast);
            if fast.is_null() {
                remove_node(slow);
                break head;
            }
            slow = advance_node(slow);
        };

        // since we should own head of list and we cannot use RC pointer
        // unsafe is the only way to change element inside and don't forget the head node

        fn advance_node(ptr: *mut Option<Box<ListNode>>) -> *mut Option<Box<ListNode>> {
            match unsafe { (*ptr).as_mut() } {
                None => { std::ptr::null_mut() }
                Some(node) => {
                    &mut node.next as *mut Option<Box<ListNode>>
                }
            }
        }

        fn remove_node(ptr: *mut Option<Box<ListNode>>) {
            use std::alloc::{Layout, dealloc};
            match unsafe { (*ptr).as_mut() } {
                None => {}
                Some(next) => {
                    let to_deallocate = &mut **next as *mut ListNode as *mut u8;
                    std::mem::swap(unsafe { &mut (*ptr) }, &mut next.next);
                    unsafe {
                        // remove useless ListNode from memory
                        dealloc(to_deallocate, Layout::new::<ListNode>());
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::iter;
    use crate::remove_nth_node_from_end_of_list::{ListNode, Solution};

    // Input: head = [1,2,3,4,5], n = 2
    // Output: [1,2,3,5]
    #[test]
    pub fn check1() {
        let input = [1, 2, 3, 4, 5];
        let n = 2;
        let expected = vec![1, 2, 3, 5];
        let head = create_linked_list(input.into_iter());

        let output = Solution::remove_nth_from_end(head, n);
        let output: Vec<_> = get_numbers(output).collect();

        assert_eq!(expected, output);
    }

    // Input: head = [1,2,3,4,5], n = 2
    // Output: [1,2,3,5]
    #[test]
    pub fn check2() {
        let input = [1, 2];
        let n = 2;
        let expected: Vec<i32> = vec![2];
        let head = create_linked_list(input.into_iter());

        let output = Solution::remove_nth_from_end(head, n);
        let output: Vec<_> = get_numbers(output).collect();

        assert_eq!(expected, output);
    }

    fn create_linked_list(input: impl DoubleEndedIterator<Item=i32>) -> Option<Box<ListNode>> {
        input.into_iter().rev().fold(None, |acc, i| {
            Some(Box::new(ListNode { val: i, next: acc }))
        })
    }

    fn get_numbers(input: Option<Box<ListNode>>) -> impl Iterator<Item=i32> {
        let mut current = input;
        iter::from_fn(move || {
            if let Some(node) = current.take() {
                let val = node.val;
                current = node.next;
                Some(val)
            } else {
                None
            }
        })
    }
}