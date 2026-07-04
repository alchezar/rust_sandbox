// IKinder

use futures::executor;
use futures::future;

pub fn main() {
    crate::show_name(file!());

    executor::block_on(say_hello());
}

async fn say_hello() {
    println!("Hello, world!");
    // second_function().await;
    futures::join!(second_function(), say_goodbye());

    let n = double(4).await;
    println!("n: {}", n);

    let futures = vec![double(1), double(2), double(3)];
    let result = future::join_all(futures).await;
    println!("result: {:?}", result);

    do_something_sync();
}

async fn second_function() {
    println!("Hello again");
}

async fn say_goodbye() {
    println!("Goodbye");
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn do_something_sync() {
    println!("Not async!");
}
