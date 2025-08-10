use shared_data::{DATA_COLLECTOR_ADDRESS, decode_v1};
use std::net::SocketAddr;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

pub async fn data_collector() -> anyhow::Result<()> {
	// Listen for TCP connections on the data collector address.
	let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

	// Loop forever, accepting connections.
	loop {
		// Wait for a new connection.
		let (socket, address) = listener.accept().await?;
		tokio::spawn(new_connection(socket, address));
	}
}

async fn new_connection(mut socket: TcpStream, address: SocketAddr) {
	println!("New connection from {:?}", address);
	let mut buf = vec![0_u8; 1024];
	loop {
		let n = socket
			.read(&mut buf)
			.await
			.expect("Failed to read data from socket.");

		if n == 0 {
			println!("No data received - connection closed.");
			return;
		}

		println!("Received {} bytes.", n);
		let received_data = decode_v1(&buf[..n]);
		println!("Received data: {:?}", received_data);
	}
}
