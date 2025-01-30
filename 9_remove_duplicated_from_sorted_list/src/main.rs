fn main() {
    let mut node_1 = ListNode::new(1);
    let mut node_2 = ListNode::new(2);
    let mut node_3 = ListNode::new(3);
    let mut node_4 = ListNode::new(3);

    node_3.next = Some(Box::new(node_4.to_owned()));
    node_2.next = Some(Box::new(node_3.to_owned()));
    node_1.next = Some(Box::new(node_2.to_owned()));

    let tc1 = Some(Box::new(node_1));
    let tc1_result = delete_duplicates(tc1);

    print_listnode(tc1_result);
}

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

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut current_node = head.as_mut();

    while let Some(node) = current_node {
        while let Some(next_node) = node.next.as_mut() {
            if next_node.val == node.val {
                node.next = next_node.next.take();
            } else {
                break;
            }
        }
        current_node = node.next.as_mut();
    }

    head
}

fn print_listnode(head: Option<Box<ListNode>>) {
    let mut current_node = head;

    let mut result = Vec::new();

    while let Some(node) = current_node {
        result.push(node.val);
        current_node = node.next;
    }

    println!("{:?}", result)
}
