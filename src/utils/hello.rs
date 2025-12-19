pub fn say_test(a: i32) {
    test(a);
}

fn test(a: i32) {
    println!("Hello, world! {}", a);
}

pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
