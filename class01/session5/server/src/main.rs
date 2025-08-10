mod collector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let handle = tokio::spawn(collector::data_collector());

	// Wait for the data collector to finish.
	handle.await??;
	Ok(())
}
