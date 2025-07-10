// IKinder

#[tokio::main]
pub async fn main() {
	crate::show_name(file!());

	tokio::select! {
		_ = do_work() => println!("wo_work finished first"),
		_ = timeout(1.0) => println!("timeout finished first"),
	}

	//
	let (tx, rx) = tokio::sync::mpsc::channel::<u32>(1);
	let (broadcast_tx, broadcast_rx) = tokio::sync::broadcast::channel::<u32>(1);

	tokio::spawn(receiver(rx, broadcast_rx));

	for count in 0..10 {
		if count % 2 == 0 {
			tx.send(count).await.unwrap();
		} else {
			broadcast_tx.send(count).unwrap();
		}
		tokio::time::sleep(std::time::Duration::from_secs(1)).await;
	}
}

async fn do_work() {
	tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}

async fn timeout(seconds: f32) {
	tokio::time::sleep(tokio::time::Duration::from_secs_f32(seconds)).await;
}

async fn receiver(mut rx: tokio::sync::mpsc::Receiver<u32>, mut broadcast_rx: tokio::sync::broadcast::Receiver<u32>) {
	loop {
		tokio::select! {
			Some(n) = rx.recv() => println!("Received message {n} on the mpsc channel"),
			Ok(n) = broadcast_rx.recv() => println!("Received message {n} on the broadcast channel"),
		}
	}
}
