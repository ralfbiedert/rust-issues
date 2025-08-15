use common::hello_from_common;

pub fn hello_from_b() {
    println!("Hello from crate B!");
    hello_from_common();
}