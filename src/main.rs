use std::{env, fs};
use std::fmt::Debug;
use std::fs::File;
use log::{error, trace};
use crate::lesson2::test;
use crate::lesson4::{obrm_usage, ref_cell_test_fail};


mod lesson1;
mod lesson2;
mod lesson3;
mod lesson4;
mod lesson5;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    // let file = File::open("foo.txt");
    // let f: File = file.unwrap();

    test();
    obrm_usage();
    ref_cell_test_fail();
}


