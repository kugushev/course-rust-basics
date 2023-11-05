pub fn test3() {
    let mut v = MyStr { num: 42 };
    let reference = &mut v;
    let moved = *reference;

    let str_ref: &'static str = &"Hello World";


    // { // lifetime 'a
    //     let mut my_struct1 = MyStruct { str: String::default() };
    //     { // lifetime 'b
    //         let mut my_struct2 = MyStruct { str: String::default() };
    //         let string: &'a String = create(&'a mut my_struct1, &'b mut my_struct2);
    //         println!("{}", string);
    //     }
    // }

}

#[derive(Copy, Clone)]
struct MyStr {
    num: i32,
}

// #[derive(Copy, Clone)]
// struct MyReference<'a> {
//     reference: &'a mut MyStr,
// }

struct MyStruct {
    str: String,
}



fn create<'a>(s1: &'a mut MyStruct, s2: &mut MyStruct) -> &'a String {
    s1.str = String::from("Hi");
    &s1.str
}
