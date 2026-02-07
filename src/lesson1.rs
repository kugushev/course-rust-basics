use std::env::var;

fn compute1(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output *= 2;
    }
    // remember that `output` will be `2` if `input > 10`
}

fn compute2(input: &u32, output: &mut u32) {
    // compiler optimisation
    let cached_input = *input;
    if cached_input > 10 {
        *output = 2;
    } else if cached_input > 5 {
        *output *= 2;
    }
}

fn run_compute(){
    // let input = 42;
    // let mut output = 0;
    // compute(&input, &mut output);
    // println!("output: {}", output); // 2
}

fn run_compute_bad(){
    // let input_output = 42;
    // compute(&input_output, &mut input_output);
    // println!("output: {}", input_output); // 2
}

fn test(){
    let mut v = vec![1];
    let _ = v.split_at_mut(1);
}

fn my_function(parameter1: String, parameter2: String) -> String {
    return format!("{} {}", parameter1, parameter2);
}

fn my_function1(parameter1: String, parameter2: String) -> String {
    format!("{} {}", parameter1, parameter2)
}

fn my_function_no_return(parameter1: String, parameter2: String) {
    println!("{} {}", parameter1, parameter2);
}

pub fn get_files_in_dir(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    for entry in std::fs::read_dir(input).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        result.push(path_str.to_string());
    }
    result
}

