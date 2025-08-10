use shared_data::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS, encode_v1};
use std::collections::VecDeque;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::time::Instant;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
	#[error("Unable to connect to the server.")]
	UnableToConnect,
	#[error("Sending the data failed.")]
	UnableToSend,
}

fn main() {
	let (tx, rx) = std::sync::mpsc::channel();

	// Start the collector thread.
	let _collector_thread = std::thread::spawn(move || {
		collect_data(tx);
	});

	// Listen for commands to send.
	let mut data_queue = VecDeque::with_capacity(120);
	while let Ok(command) = rx.recv() {
		let encoded = encode_v1(&command);
		data_queue.push_back(encoded);

		// Send all the queue commands.
		if send_queue(&mut data_queue).is_err() {
			println!("Error sending command.");
			break;
		}
	}
}

pub fn collect_data(tx: Sender<CollectorCommandV1>) {
	// Initialize the sysinfo system.
	let mut sys = sysinfo::System::new_all();

	// Perform a single refresh and pause. `sysinfo` gathers data via deltas,
	// and the first reading is usually useless.
	sys.refresh_memory();
	sys.refresh_cpu_all();
	std::thread::sleep(std::time::Duration::from_secs(1));

	// Run forever
	loop {
		// Note when we're starting.
		let now = Instant::now();

		// Refresh the stored data.
		sys.refresh_memory();
		sys.refresh_cpu_all();

		// Get new values.
		let total_memory = sys.total_memory();
		let used_memory = sys.used_memory();
		let num_cpus = sys.cpus().len();
		let total_cpu_usage = sys
			.cpus()
			.iter()
			.map(|cpu| cpu.cpu_usage())
			.sum::<f32>();
		let average_cpu_usage = total_cpu_usage / num_cpus as f32;

		// Submit.
		let send_result = tx.send(CollectorCommandV1::SubmitData {
			collector_id: 0,
			total_memory,
			used_memory,
			average_cpu_usage,
		});
		if let Err(e) = send_result {
			println!("Error sending data: {e:?}");
		}

		// Wait for the next cycle.
		let elapsed_seconds = now.elapsed().as_secs_f32();
		if elapsed_seconds < 1.0 {
			std::thread::sleep(std::time::Duration::from_secs_f32(1.0 - elapsed_seconds));
		} else {
			// Warning: we're running behind!
			std::thread::sleep(std::time::Duration::from_secs(1));
		}
	}
}

pub fn send_command(command: &Vec<u8>) -> Result<(), CollectorError> {
	println!("Encoded {} bytes", command.len());
	let mut stream = TcpStream::connect(DATA_COLLECTOR_ADDRESS).map_err(|_| CollectorError::UnableToConnect)?;
	stream
		.write_all(&command)
		.map_err(|_| CollectorError::UnableToSend)?;

	Ok(())
}

pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
	// Connect.
	let mut stream = TcpStream::connect(DATA_COLLECTOR_ADDRESS).map_err(|_| CollectorError::UnableToConnect)?;

	// Send every queue item.
	while let Some(command) = queue.pop_front() {
		println!("Encoded {} bytes", command.len());
		if stream.write_all(&command).is_err() {
			queue.push_front(command);
			return Err(CollectorError::UnableToSend);
		}
	}

	Ok(())
}
