fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn longest<'a>(a: &'a str, b: &str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn longest<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}