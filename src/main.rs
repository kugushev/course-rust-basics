use std::{env, fs};
use std::fmt::Debug;
use std::fs::File;
use log::{error, trace};
use crate::lesson2::test;


mod lesson1;
mod lesson2;
mod lesson3;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    // let file = File::open("foo.txt");
    // let f: File = file.unwrap();

    test();

}


