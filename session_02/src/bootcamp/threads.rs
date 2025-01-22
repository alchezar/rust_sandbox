// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_macros)]

use tokio_stream::StreamExt;

#[derive(Debug)]
struct Database {
	connections: Vec<u32>,
}

impl Database {
	fn new() -> Database {
		Database { connections: vec![] }
	}
	fn connect(&mut self, id: u32) {
		self.connections.push(id);
	}
}

#[tokio::main(flavor = "current_thread")]
pub async fn run() {
	println!("--- Async ---");
	let f = async_func(666);
	println!("Let's get rusty!");
	f.await;

	println!("--- Tokio ---");
	let mut handles = vec![];

	for i in 0..2 {
		let handle = tokio::spawn(async move { async_func(i).await });
		handles.push(handle);
	}

	handles.push(tokio::spawn(async {
		let _res = tokio::task::spawn_blocking(|| {
			expensive_computations();
		})
		.await;
	}));

	for h in handles {
		h.await.unwrap();
	}

	println!("--- Streams ---");
	let mut stream = tokio_stream::iter(["let's", "Get", "Rusty"]).map(|s| s.to_ascii_uppercase());
	while let Some(s) = stream.next().await {
		println!("{}", s);
	}
}

fn expensive_computations() {
	let mut i = 0;
	for _ in 0..400_000_000 {
		i += 1;
	}
	println!("Done with expensive computations i = {i}")
}

async fn async_func(i: i32) {
	println!("[{i}] I'm an async function!");
	let s1 = read_from_database().await;
	println!("[{i}] First result : {s1}");
	let s2 = read_from_database().await;
	println!("[{i}] Second result : {s2}");
}

async fn read_from_database() -> String {
	tokio::time::sleep(std::time::Duration::from_millis(10)).await;
	"DB result".to_owned()
}

fn database_connection() {
	let db = std::sync::Arc::new(std::sync::Mutex::new(Database::new()));
	let mut handles = vec![];

	for i in 0..10 {
		let db = std::sync::Arc::clone(&db);
		let handle = std::thread::spawn(move || {
			let mut db_lock = db.lock().unwrap();
			db_lock.connect(i)
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}

	println!("{:?}", *db.lock().unwrap())
}

fn revert_strings() {
	let (tx, rx) = std::sync::mpsc::channel();

	let sentences = [
		"!dlroW.olleH".to_owned(),
		".tsurT eW tsuR nI".to_owned(),
		"!tsuR teG s'teL".to_owned(),
		"!tsuB ro tsuR".to_owned(),
	];

	for s in sentences {
		let tx_clone = tx.clone();

		std::thread::spawn(move || {
			let reverted: String = s.chars().rev().collect();
			tx_clone.send(reverted).unwrap();
		});
	}

	drop(tx);
	for sentence in rx {
		println!("{}", sentence);
	}
}
