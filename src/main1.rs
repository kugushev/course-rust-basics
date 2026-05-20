struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };

    let name = user.name;
    println!("{}", user.age);
    println!("{}", user.name);
}