use common::hello_from_common;

pub fn hello_from_a() {
    println!("Hello from crate A!");
    hello_from_common();
}