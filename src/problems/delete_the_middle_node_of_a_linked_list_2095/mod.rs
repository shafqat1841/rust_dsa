// delete_the_middle_node_of_a_linked_list_2095

#![allow(dead_code)]
#![allow(unused)]


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

fn sol1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    None
}

struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        sol1(head)
    }
}

fn get_input() -> Option<Box<ListNode>> {
    let mut ln1 = ListNode::new(1);
    let mut ln2 = ListNode::new(3);
    let mut ln3 = ListNode::new(4);
    let mut ln4 = ListNode::new(7);
    let mut ln5 = ListNode::new(1);
    let mut ln6 = ListNode::new(2);
    let mut ln7 = ListNode::new(6);
    
    ln6.next = Some(Box::new(ln7));
    ln5.next = Some(Box::new(ln6));
    ln4.next = Some(Box::new(ln5));
    ln3.next = Some(Box::new(ln4));
    ln2.next = Some(Box::new(ln3));
    ln1.next = Some(Box::new(ln2));

    Some(Box::new(ln1))
}

fn get_output() -> Option<Box<ListNode>> {
    let mut ln1 = ListNode::new(1);
    let mut ln2 = ListNode::new(3);    
    let mut ln3 = ListNode::new(4);
    let mut ln4 = ListNode::new(1);
    let mut ln5 = ListNode::new(2);
    let mut ln6 = ListNode::new(6);
    
    ln5.next = Some(Box::new(ln6));
    ln4.next = Some(Box::new(ln5));
    ln3.next = Some(Box::new(ln4));
    ln2.next = Some(Box::new(ln3));
    ln1.next = Some(Box::new(ln2));

    Some(Box::new(ln1))
}

#[cfg(test)]
mod delete_the_middle_node_of_a_linked_list_2095_tests {
    use super::*;


    #[test]
    fn main() {

        // Input: head = [1,3,4,7,1,2,6]
        // Output: [1,3,4,1,2,6]  
        
        let input = get_input();        
        let output = get_output();        
        let res = Solution::delete_middle(input);
        assert_eq!(output, res);

    }

}
