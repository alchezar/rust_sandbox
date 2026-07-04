// IKinder

use rayon::prelude::*;

pub fn main() {
    crate::show_name(file!());

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.join(test, test);

    //broadcast(pool);
    //scoped_pool(pool);
}

fn test() {
    println!("test");
}

fn broadcast(pool: rayon::ThreadPool) {
    pool.scope(|scope| {
        scope.spawn_broadcast(|_scope, context| {
            println!("Hello from broadcast thread {}", context.index())
        });
    });
}

fn scoped_pool(pool: rayon::ThreadPool) {
    pool.spawn(|| println!("Hello from pool thread!"));
    pool.scope(|scope| {
        for n in 0..20 {
            scope.spawn(move |_| {
                println!("Hello from scoped thread {}", n);
            });
        }
    });
    println!("Hello from main thread!");
}
