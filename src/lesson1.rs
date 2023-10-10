
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
    // let's draw
    let cached_input = *input; // keep `*input` in a register
    if cached_input > 10 {
        *output = 2;
    } else if cached_input > 5 {
        *output *= 2;
    }
}

fn test(){
    let mut v = vec![];
    v.split_at_mut(1);
}

fn my_function(parameter1: String, parameter2: String) -> String {
    return format!("{} {}", parameter1, parameter2);
}

fn my_function(parameter1: String, parameter2: String) -> String {
    format!("{} {}", parameter1, parameter2)
}

fn my_function_no_return(parameter1: String, parameter2: String) {
    println!("{} {}", parameter1, parameter2);
}

