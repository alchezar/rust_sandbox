// IKinder

use std::sync::Mutex;

static SHARED: Mutex<u32> = Mutex::new(0);

pub fn main() {
	crate::show_name(file!());

	//deadlock();
	poisoning();
}

pub fn poisoning() {
	let handle = std::thread::Builder::new()
		.name("Minion".to_owned())
		.spawn(poisoned)
		.unwrap();
	println!("Trying to return from the thread");
	println!("On join: {:?}", handle.join());

	let lock = SHARED.lock();
	// match lock {
	// 	Ok(data) => println!("Data: {}", data),
	// 	Err(err) => println!("Rec data: {}", err.into_inner()),
	// }
	println!("Locking attempt: {:?}", lock);

	let recovered_data = lock.unwrap_or_else(|poisoned| {
		println!("Mutex was poisoned, recovering data...");
		poisoned.into_inner()
	});
	println!("Recovered data: {:?}", recovered_data);
}

pub fn deadlock() {
	println!("::deadlock\n");

	let my_shared = Mutex::new(0);
	let lock = SHARED.lock().unwrap();
	drop(lock);
	if let Ok(lock) = SHARED.try_lock() {
		println!("Got the lock");
	} else {
		println!("No luck");
	}
}

fn poisoned() {
	let mut lock = SHARED.lock().unwrap();
	*lock += 1;
	panic!("The poisoner strikes")
}
