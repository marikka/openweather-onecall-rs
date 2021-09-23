use openweather_onecall::{Coords, OpenWeatherClient};

fn main() {
    let api_key = std::env::var("OPENWEATHER_API_KEY").unwrap();
    let coords = Coords {
        lat: 40.7128,
        lon: -74.0060,
    };
    let client = OpenWeatherClient::new(&api_key);
    let json = client.onecall(&coords);

    println!("{:?}", json);
}
