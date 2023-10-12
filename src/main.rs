use std::{env, fs};
use std::fs::File;


mod lesson1;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    let file = File::open("foo.txt");
    let f: File = file.unwrap();
}


