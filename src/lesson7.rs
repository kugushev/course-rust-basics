use std::{array, cmp};
use std::cmp::Ordering;
use std::slice::Iter;

fn slice1() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
    let slice: &[i32] = &vec[2..5]; // 3
    drop(vec);
    // println!("{:?}", slice);
}

fn split() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7];
    let (left, right): (&[i32], &[i32]) = vec.split_at(3);
    println!("{:?} {:?}", left, right);
}

fn boxed_slice() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let boxed: Box<[i32]> = vec.into_boxed_slice();
    let slice = &boxed[2..5];
    println!("{:?}", slice);
}

pub fn test_slice() {
    let mut vec = vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'];
    let slice = &vec[6..11];
    println!("{:?}", slice);
    let (hello, mut world) = vec.split_at(5);
    world = &world[1..];
    println!("{:?} {:?}", hello, world);
    let mut slice_mut =  &mut vec[6..11];
    slice_mut[0] = 's';
    slice_mut[1] = 'p';
    slice_mut[2] = 'a';
    slice_mut[3] = 'c';
    slice_mut[4] = 'e';

    let slice = &vec[6..11];
    println!("{:?}", slice);
}

pub fn test_str() {
    // let mut vec = String::from("привет мир");
    // let slice = &vec[6..11];
    // println!("{:?}", slice);
    // let (hello, mut world) = vec.split_at(5);
    // world = &world[1..];
    // println!("{:?} {:?}", hello, world);
    //
    //
    // let slice = &vec[6..11];
    // println!("{:?}", slice);
}

pub fn test_iter() {
    // cheat-sheet https://danielkeep.github.io/itercheat_baked.html

    // let nums0to10 = create_nums(0..10);
    // let nums100to110 = create_nums(100..110);
    //
    // let even_numbers: Vec<String> = // get even numbers in both ranges
    //
    // assert_eq!(even_numbers, vec!["2", "4", "6", "8", "100", "102", "104", "106", "108"]);
    //
    // let chars_not_0: String = even_numbers. // get chars of even_numbers without 0
    //
    // assert_eq!(chars_not_0, String::from("2468112141618"));
    //
    // let nums0to10 = create_nums(0..10);
    // let nums100to110 = create_nums(100..110);
    //
    // let summ: i32 = // calculate the following expression using nums0to10 and nums100to110 iterators
    // assert_eq!(summ, (3 * 103) + (4 * 104))
}

// fn create_nums(range: Range<i32>) -> impl Iterator<Item=i32> {
//     // you should implement generator of iterator from range
// }

fn arrrr(){
    // let arr = [0, 1, 2, 3, 4];
    // let s: [i32] = arr[..];
    // let i = 1;
    // let b = arr[i..];



}

// struct Vec<T,A: Allocator = Global> {
//     buf: RawVec<T, A>,
//     len: usize,
// }
//
// struct RawVec<T, A: Allocator = Global> {
//     ptr: Unique<T>,
//     cap: usize,
//     alloc: A,
// }
//
// struct Unique<T: ?Sized> {
//     pointer: NonNull<T>,
//     _marker: PhantomData<T>,
// }

// pub struct Vec<T> {
//     ptr: NonZero<*mut T>,
//     len: uint,
//     cap: uint,
// }


struct Fibonacci {
    curr: u32,
    next: u32,
}


impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

// pub trait IntoIterator {
//     type Item;
//     type IntoIter: Iterator<Item = Self::Item>;
//     fn into_iter(self) -> Self::IntoIter;
// }


fn main() {
    // let v = vec![];
// v.into_iter()
    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}


// pub trait Iterator {
//     /// The type of the elements being iterated over.
//     type Item;
//
//     /// Advances the iterator and returns the next value.
//     fn next(&mut self) -> Option<Self::Item>;
// }

struct BufferOf8<T>([Option<T>; 8]);

impl<T> BufferOf8<T>{
    pub fn iter(&self) -> BufferOf8Iter<T>{
        BufferOf8Iter {
            reference: self,
            next: 0
        }
    }
}

struct BufferOf8Iter<'a, T> {
    reference: &'a BufferOf8<T>,
    next: usize
}

impl<'a, T> Iterator for BufferOf8Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.reference.0.get(self.next);
        self.next += 1;
        match next {
            Some(Some(itm)) => { Some(itm) }
            _ => None
        }
    }
}

pub fn test_iter_cust(){
    let buffer = BufferOf8([Some(1), Some(2), Some(3), None, None, None, None, None]);
    let strings: Vec<_> = buffer.iter().filter(|i|  **i >= 2).map(|i| i.to_string()).collect(); // 2 3
    for s in strings {
        println!("{}", s)
    }
}