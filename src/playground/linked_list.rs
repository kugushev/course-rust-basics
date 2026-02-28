struct Node<'a> {
    value: i32,
    next: Option<&'a Node<'a>>
}

pub fn test_linked_list() {
    let mut node1 = Node { value: 1, next: None };
    let mut node2 = Node { value: 2, next: None };
    let mut node3 = Node { value: 3, next: None };

    node2.next = Some(&node3);
    node1.next = Some(&node2);
    
    println!("{}", node1.value);
    println!("{}", node1.next.unwrap().value);
    println!("{}", node1.next.unwrap().next.unwrap().value);
}