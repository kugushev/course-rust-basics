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
use std::ops::Deref;

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