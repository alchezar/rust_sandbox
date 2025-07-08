// IKinder

use std::time::Duration;

#[tokio::main]
pub async fn main() {
	crate::show_name(file!());

	tokio::join!(hello_delay(1, 500), hello_delay(2, 1000), hello_delay(3, 1500));
}

async fn hello_delay(task: u64, time: u64) {
	println!("Task {task} has started");

	// 1. Bad idea. The thread with tokio runtime will go to sleep too.
	// std::thread::sleep(Duration::from_millis(time));

	// 2. Tokio's mirrored version, to achieve correct thread sleep behavior.
	// tokio::time::sleep(Duration::from_millis(time)).await;

	// 3. Or use spawn blocking, to use threads, where blocking is acceptable.
	let _ = tokio::task::spawn_blocking(move || {
		std::thread::sleep(Duration::from_millis(time));
	});

	println!("Task {task} has finished");
}
