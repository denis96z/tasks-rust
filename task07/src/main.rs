use std::{thread, time};
use futures::executor::block_on;

fn main() {
    let f = print_sum(1, 2);
    println!("main");
    block_on(f);
}

async fn print_sum(x1: i32, x2: i32) {
    let y = sum(x1, x2).await;
    println!("Y = {} + {} = {}", x1, x2, y);
}

async fn sum(x1: i32, x2: i32) -> i32 {
    thread::sleep(time::Duration::from_secs(1));
    x1 + x2
}
