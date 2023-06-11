use std::env;
use structopt::StructOpt;
use serde::Deserialize;
use reqwest::Error;

#[derive(StructOpt)]
struct Cli {
    city: String,
}

#[derive(Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::from_args();
    let api_key = env::var("OPENWEATHERMAP_API_KEY").expect("OPENWEATHERMAP_API_KEY not set");

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        args.city, api_key
    );

    let response = reqwest::get(&url).await?.json::<WeatherResponse>().await?;

    println!("Weather in {}: {} and temperature is {}°C", args.city, response.weather[0].description, response.main.temp);

    Ok(())
}
