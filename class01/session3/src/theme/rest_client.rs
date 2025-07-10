// IKinder

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
	crate::show_name(file!());

	const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=49.8429&longitude=24.0311&current_weather=true";
	let response = reqwest::get(URL).await?;
	// let weather = response.text().await?;
	// println!("{:?}", weather);

	let weather = response
		.json::<serde_json::Value>()
		.await?;
	println!("{:#?}", weather);

	Ok(())
}

struct Weather {
	latitude: f64,
	longitude: f64,
	current_weather: CurrentWeather,
}

#[derive(serde::Deserialize, Debug)]
struct CurrentWeather {
	temperature: f64,
	wind_speed: f64,
}
