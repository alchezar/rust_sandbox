use class10::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:6666").map_err(anyhow::Error::from)?;
	hc.do_get("/hello")
		.await
		.map_err(anyhow::Error::from)?
		.print()
		.await
		.map_err(anyhow::Error::from)?;
	Ok(())
}
