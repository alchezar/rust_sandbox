// IKinder

use async_recursion::*;
use futures::future::{BoxFuture, FutureExt};
use std::pin::Pin;

#[tokio::main]
pub async fn main() {
    crate::show_name(file!());

    println!("Fibonacci 10 = {}", fibonacci_async(10).await);

    let mut future = async {
        println!("Hello World");
    };
    tokio::pin!(future);
    (&mut future).await;
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[async_recursion]
async fn fibonacci_async(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_async(n - 1).await + fibonacci_async(n - 2).await,
    }
}

async fn one() {
    println!("one");
}

async fn two() {
    println!("two");
}

async fn call_one_of_them(n: u32) -> Pin<Box<dyn Future<Output = ()>>> {
    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("Invalid choice"),
    }
}
