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
use std::cell::{Cell, Ref, RefCell, RefMut, UnsafeCell};
use std::intrinsics::drop_in_place;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;

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

fn test_box_s() {
    let mut boxed = Box::new(MyStruct { my_value: 42 });
    let mut x: MyStruct = *boxed;
    x.my_value = 123;
    println!("{}", x.my_value);
    let reference = &x;
    // println!("{}", boxed.my_value)
}

fn dyn_arr(size: usize) {
    let v = vec![1, 2, 3];
    let b = v.into_boxed_slice();
}

fn rc_test() {
    let owner1 = Rc::new(MyStruct { my_value: 42 });
    let owner2: Rc<MyStruct> = owner1.clone();
    let owner3: Rc<MyStruct> = owner2.clone();
    println!("{}", owner3.my_value);
    println!("{}", owner1.my_value);
    println!("{}", owner2.my_value);
    let reference: &MyStruct = &owner1;
    println!("{}", reference.my_value);

    // Rc::downgrade().upgrade()
}

fn cell_test() {
    let cell = Cell::new(42);
    let reference1: &Cell<i32> = &cell;
    let reference2: &Cell<i32> = &cell;
    println!("{}", reference1.get()); // > 42
    reference1.replace(123);
    println!("{}", reference2.get()); // > 123
}

fn ref_cell_test() {
    let cell = RefCell::new(MyStruct { my_value: 42 });
    {
        let reference1: Ref<MyStruct> = cell.borrow();
        println!("{}", reference1.my_value); // > 42
    }
    {
        let mut borrowed: RefMut<MyStruct> = cell.borrow_mut();
        borrowed.my_value = 123;
    }

    let reference2: Ref<MyStruct> = cell.borrow();
    println!("{}", reference2.my_value); // > 123
}

pub fn ref_cell_test_fail() {
    let cell = RefCell::new(MyStruct { my_value: 42 });

    let reference1: Ref<MyStruct> = cell.borrow();
    println!("{}", reference1.my_value); // > 42

    {
        let mut borrowed: RefMut<MyStruct> = cell.borrow_mut();
        borrowed.my_value = 123;
    }

    let reference2: Ref<MyStruct> = cell.borrow();
    println!("{}", reference2.my_value); // > 123
}

// impl<T: Copy> Cell<T> {
//     pub fn get(&self) -> T {
//         unsafe { *self.value.get() }
//     }
// }

// impl<T: ?Sized> UnsafeCell<T> {
//     pub const fn get(&self) -> *mut T {
//         self as *const UnsafeCell<T> as *const T as *mut T
//     }
// }

// struct RefCell<T: ?Sized> {
//     borrow: Cell<BorrowFlag>,
//     borrowed_at: Cell<Option<&'static crate::panic::Location<'static>>>,
//     value: UnsafeCell<T>,
// }

// unsafe impl<T: ?Sized, A: Allocator> Drop for Rc<T, A> {
//     fn drop(&mut self) {
//         unsafe {
//             self.inner().dec_strong();
//             if self.inner().strong() == 0 {
//                 // destroy the contained object
//                 ptr::drop_in_place(Self::get_mut_unchecked(self));
//
//                 // remove the implicit "strong weak" pointer now that we've
//                 // destroyed the contents.
//                 self.inner().dec_weak();
//
//                 if self.inner().weak() == 0 {
//                     self.alloc.deallocate(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
//                 }
//             }
//         }
//     }
// }

