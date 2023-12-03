use std::ops::Range;

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
    let mut vec = String::from("привет мир");
    let slice = &vec[6..11];
    println!("{:?}", slice);
    let (hello, mut world) = vec.split_at(5);
    world = &world[1..];
    println!("{:?} {:?}", hello, world);


    let slice = &vec[6..11];
    println!("{:?}", slice);
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
    let arr = [0, 1, 2, 3, 4];
    let s: [i32] = arr[..];
    let i = 1;
    let b = arr[i..];

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
