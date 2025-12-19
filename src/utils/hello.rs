use crate::enums::ip_addr_kind::IpAddrKind;
pub fn say_test(a: i32) {
    test(a);
}

fn test(a: i32) {
    println!("Hello, world! {}", a);
}

pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}


pub fn ip(ip:IpAddrKind) -> String {
    match ip {
        IpAddrKind::V4 => format!("IPv4:{:?}",ip),
        IpAddrKind::V6 => format!("IPv6:{:?}",ip),
    }
}