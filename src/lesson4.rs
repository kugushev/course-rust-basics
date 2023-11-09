// struct User {
//     first_name: String,
//     second_name: String,
//     middle_name: String,
// }
//
// fn print(user: &User) {
//     println!("{} {} {}", user.first_name, user.second_name, user.middle_name);
// }

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::intrinsics::drop_in_place;
use std::ops::Deref;
use std::borrow::Borrow;

struct User {
    first_name: String,
    second_name: String,
    middle_name: Option<String>,
}

struct MyStruct {
    my_value: i32,
}

struct MyObrm<T> {
    ptr: *mut T,
}

impl<T> MyObrm<T> {
    fn new(val: T) -> Self {
        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout) as *mut T;
            if ptr.is_null() {
                handle_alloc_error(layout);
            }

            *ptr = val;

            MyObrm { ptr }
        }
    }
}

pub fn obrm_usage() {
    let user = MyObrm::new(MyStruct {
        my_value: 42
    });

    println!("Print: {}", user.my_value);

    drop(user);
}

impl<T> Deref for MyObrm<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
        }
    }
}

impl<T> Drop for MyObrm<T> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::new::<T>();
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}

struct MyStr {
    int: i32,
}

struct MyStr2 {
    int: i128,
}

fn test123() {
    // let mut str = MyStr { int: 42 };
    //
    // drop(str);
    //
    // str = MyStr2 { int: 213123123 };
}

// struct Node {
//     value: i32,
//     next: Option<Node>,
// }


// struct Node<'a> {
//     value: i32,
//     next: Option<&'a Node<'a>>,
// }
//
//
// fn test_ll() {
//     let mut node1 = Node { value: 1, next: None };
//     {
//         let mut node2 = Node { value: 2, next: None };
//         let mut node3 = Node { value: 3, next: None };
//
//         node2.next = Some(&node3);
//         node1.next = Some(&node2);
//
//         node1.next = None;
//     }
//
//     // println!("{}", node1.value)
// }

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}


fn test_ll() {
    let mut node1 = Node { value: 1, next: None };
    {
        let mut node2 = Node { value: 2, next: None };
        let mut node3 = Node { value: 3, next: None };

        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));

        node1.next = None;
    }

    println!("{}", node1.value)
}


fn test_box() {
    let mut boxed = Box::new(MyStruct { my_value: 42 });
    let reference: &MyStruct = &boxed;
    println!("{}", reference.my_value);
    let mut_reference: &mut MyStruct = &mut boxed;
    mut_reference.my_value = 123;
    let reference: &Box<MyStruct> = &boxed;
    println!("{}", reference.my_value);
}

fn test_box_s(){
    let mut boxed = Box::new(MyStruct { my_value: 42 });
    let mut x: MyStruct = *boxed;
    x.my_value = 123;
    println!("{}", x.my_value);
    let reference = &x;
    println!("{}", boxed.my_value)

}