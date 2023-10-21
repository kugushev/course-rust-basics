use std::fmt::Debug;


fn test() {
    let tuple = (42, "hello", true);
    my_trace(tuple);

    let (error, message) = go_like_function(42);
    if !error {
        eprint!("Error: {}", message);
    }

    let mut my_struct = MyStruct::new(42);
    my_struct = my_struct.update_and_print(10);
    MyStruct::static_method();


    MyStruct::static_trait_method();

    generic_fn::<MyStruct>();

    (true, 42).print_and_clean();
    [MyStruct::new(42), MyStruct::new(42)].print_and_clean();

    let mut vec: Vec2;
    vec = 42.to_vec2();
    vec = (42, 42).to_vec2();
    vec = [42, 42].to_vec2();
}

#[derive(Copy, Clone)]
struct MyStruct {
    num: i32,
}

impl MyStruct {
    fn new(num: i32) -> Self {
        Self { num }
    }

    fn static_method() {
        println!("My Struct is good");
    }

    fn update_and_print(mut self, increment: i32) -> Self {
        self.num += increment;
        println!("{}", self.num);
        self
    }
}

struct MyOtherStruct {
    str: MyStruct,
}

fn my_trace<T: Debug>(item: T) {
    println!("{:?}", item);
}


fn go_like_function(age: i32) -> (bool, String) {
    if age > 0 && age < 120 {
        (true, String::from("You are alive!"))
    } else {
        (false, String::from("You are not alive!"))
    }
}

trait StaticTrait {
    fn static_trait_method();
}

impl StaticTrait for MyStruct {
    fn static_trait_method() {
        println!("My Struct is good");
    }
}

fn generic_fn<T: StaticTrait>() {
    T::static_trait_method();
}


trait MyTrait {
    fn print_and_clean(self) -> Self;
}

impl MyTrait for MyStruct {
    fn print_and_clean(mut self) -> Self {
        println!("{}", self.num);
        self.num = 0;
        self
    }
}

impl MyTrait for (bool, i32) {
    fn print_and_clean(mut self) -> Self {
        println!("{}", self.1);
        self.0 = false;
        self.1 = 0;
        self
    }
}

impl MyTrait for [MyStruct; 2] {
    fn print_and_clean(mut self) -> Self {
        self[0].print_and_clean();
        self[1].print_and_clean();
        self
    }
}

trait AnswerOnTheQuestion {
    fn answer_on_the_question() -> i32;
}

impl AnswerOnTheQuestion for MyStruct {
    fn answer_on_the_question() -> i32 {
        42
    }
}

trait Power {
    fn amount(self) -> i32;
    fn is_powerful(self) -> bool where Self: Sized {
        self.amount() > 9000
    }
}

impl Power for i32 {
    fn amount(self) -> i32 {
        self
    }
}

impl Power for bool {
    fn amount(self) -> i32 {
        if self { i32::MAX } else { i32::MIN }
    }

    fn is_powerful(self) -> bool {
        self
    }
}

struct Vec2(i32, i32);

trait Convert {
    fn to_vec2(self) -> Vec2;
}

impl Convert for i32 {
    fn to_vec2(self) -> Vec2 {
        Vec2(self, self)
    }
}

impl Convert for (i32, i32) {
    fn to_vec2(self) -> Vec2 {
        Vec2(self.0, self.1)
    }
}

impl Convert for [i32; 2] {
    fn to_vec2(self) -> Vec2 {
        Vec2(self[0], self[1])
    }
}