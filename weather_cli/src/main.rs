use std::env;
// (!) StructOpt is a crate that allows us to parse command line arguments
use structopt::StructOpt;
// (!) serde is a crate that allows us to deserialize JSON
use serde::Deserialize;
use reqwest::Error;

// (!) derive is a way to apply a trait to a struct. This is done
// through what is a called a "procedural macro".
#[derive(StructOpt)]
struct Cli {
    city: String,
}

// (!) Deserialize is a trait that allows us to deserialize JSON
#[derive(Deserialize)]
struct Main {
    temp: f32,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

#[derive(Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
}

macro_rules! customLog {
    () => {
        // The macro will expand into the contents of this block.
        println!("Welcome to the Weather CLI!")
    };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    customLog!();

    let args = Cli::from_args();
    let api_key = env::var("OPENWEATHERMAP_API_KEY").expect("OPENWEATHERMAP_API_KEY not set");

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        args.city, api_key
    );

    let response = reqwest::get(&url).await?.json::<WeatherResponse>().await?;

    // (!) println! is a macro created through macro_rules!
    println!("Weather in {}: {} and temperature is {}°C", args.city, response.weather[0].description, response.main.temp);

    Ok(())
}
