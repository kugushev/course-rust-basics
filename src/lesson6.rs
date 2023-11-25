fn main() {
    println!("Hello, world!");

    execute1(|i| i.to_string());

    let mut my_string = String::from("something");

    execute2(|| my_string.len());

    execute3(|s| my_string.push_str(&s));

    execute4(|| my_string);
}

fn execute1(func: fn(i32) -> String) {
    let result = func(42);
    assert_eq!("42", result);
}

fn execute2(func: impl Fn() -> usize) {
    let string_len = func();
    assert_eq!(9, string_len)
}

fn execute3(mut func: impl FnMut(String)) {
    func(String::from(" good"));
}

fn execute4(func: impl FnOnce() -> String) {
    let result: String = func();
    assert_eq!("something good", result);
}