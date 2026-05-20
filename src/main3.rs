mod longest;
mod main;

fn main() {
    let s = String::from("hello");
    let f = move || {
        let s_ref: &String = &s;
        println!("{}", s_ref);
    };
    println!("{}", s);
    f();
}