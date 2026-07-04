// IKinder

use tokio::runtime::Builder;

pub fn main() {
    crate::show_name(file!());

    // main1();
    // main2();
    main3();
    // main4();
}

fn main1() {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .build()
        .unwrap();
    runtime.block_on(hello());
}

#[tokio::main(flavor = "current_thread")]
async fn main2() {
    hello().await;
    let result = tokio::join!(hello(), hello2());
    println!("{:?}", result);
}

#[tokio::main]
async fn main3() {
    tokio::spawn(ticker()).await.unwrap();
    hello().await;
}

#[tokio::main(flavor = "current_thread")]
async fn main4() {
    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker())
    );
    println!("Finished!");
}

async fn hello() -> u32 {
    println!("Hello Tokio");
    3
}

async fn hello2() -> u32 {
    println!("Hello Tokio2");
    4
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {}", i);
        tokio::task::yield_now().await;
    }
}
