use std::ops::Deref;

struct Node{
    value: i32,
    next: Option<Box<Node>>
}

pub fn test_linked_list_heap(){
    let node3 = Node{value: 3, next: None};
    let node2 = Node{value: 2, next: Some(Box::new(node3))};
    let node1 = Node{value: 1, next: Some(Box::new(node2))};
    
    println!("{}", node1.value);
    let node1_next: Option<&Box<Node>> = node1.next.as_ref();
    println!("{}", node1_next.unwrap().value);
    println!("{}", node1.next.as_ref().unwrap().next.as_ref().unwrap().value);
}