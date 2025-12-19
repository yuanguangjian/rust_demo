mod model;
mod utils;
// 声明 utils 目录作为模块
use model::user::User;
use utils::hello; // 引用 hello.rs 里的函数
fn main() {
    println!("你好");
    let a = 20;
    hello::say_test(a);
    {
        let a = 30;
        println!("a is {0}, a again is {0}", a);
    }

    let mut name = "袁小建";
    println!("{}", name);
    name = "hello";
    println!("{}", name);

    let a = 10;
    let mut b: i32 = 12;

    let a = hello::add(a, b);

    println!("{}", a);

    let a: bool = false;

    println!("{}", a);

    if a {
        println!("正确");
    } else {
        println!("错误")
    }

    if b == 12 {
        println!("xxx")
    }

    let mut num = 1;

    loop {
        println!("{}", num);
        num += 1;
        if num == 10 {
            break;
        }
    }

    println!("{}", num);

    while num != 0 {
        println!("{}", num);
        num -= 1;
    }
    println!("{}", num);

    for i in 1..4 {
        print!("{}", i);
    }

    let u = User {
        userName: String::from("hello"),
        userId: 20,
        deleted: false,
        age: 30,
    };

    println!("{}", serde_json::to_string(&u).unwrap());

}
