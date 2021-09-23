//! Crate implementing the OpenWeather One Call API

use chrono::{serde::ts_seconds, DateTime, Utc};
use reqwest::blocking::get;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OnecallResponse {
    lat: f32,
    lon: f32,
    timezone: String,
    timezone_offset: i32,
    current: Current,
    minutely: Vec<Minutely>,
    hourly: Vec<Hourly>,
    daily: Vec<Daily>,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Rain {
    #[serde(rename = "1h")]
    one_hour: f32,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    #[serde(with = "ts_seconds")]
    dt: DateTime<Utc>,
    sunrise: u32,
    sunset: u32,
    temp: f32,
    feels_like: f32,
    pressure: u32,
    humidity: u32,
    dew_point: f32,
    uvi: f32,
    clouds: u32,
    visibility: u32,
    wind_speed: f32,
    wind_deg: u32,
    wind_gust: Option<f32>,
    weather: Vec<Weather>,
    rain: Option<Rain>,
    alerts: Option<Vec<Alert>>,
}

#[derive(Deserialize, Debug)]
pub struct Minutely {
    #[serde(with = "ts_seconds")]
    dt: DateTime<Utc>,
    precipitation: u32,
}

#[derive(Deserialize, Debug)]
pub struct Hourly {
    #[serde(with = "ts_seconds")]
    dt: DateTime<Utc>,
    temp: f32,
    feels_like: f32,
    pressure: u32,
    humidity: u32,
    dew_point: f32,
    uvi: f32,
    clouds: u32,
    visibility: u32,
    wind_speed: f32,
    wind_deg: u32,
    wind_gust: f32,
    weather: Vec<Weather>,
    pop: f32,
    rain: Option<Rain>,
}

#[derive(Deserialize, Debug)]
pub struct Temp {
    day: f32,
    min: f32,
    max: f32,
    night: f32,
    eve: f32,
    morn: f32,
}

#[derive(Deserialize, Debug)]
pub struct FeelsLike {
    day: f32,
    night: f32,
    eve: f32,
    morn: f32,
}

#[derive(Deserialize, Debug)]
pub struct Daily {
    #[serde(with = "ts_seconds")]
    dt: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    sunrise: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    sunset: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    moonrise: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    moonset: DateTime<Utc>,
    moon_phase: f32,
    temp: Temp,
    feels_like: FeelsLike,
    pressure: u32,
    humidity: u32,
    dew_point: f32,
    wind_speed: f32,
    wind_deg: u32,
    wind_gust: f32,
    weather: Vec<Weather>,
    clouds: u32,
    pop: f32,
    rain: Option<f32>,
    uvi: f32,
}

#[derive(Deserialize, Debug)]
pub struct Alert {
    sender_name: String,
    event: String,
    #[serde(with = "ts_seconds")]
    start: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    end: DateTime<Utc>,
    description: String,
    tags: Vec<String>,
}

pub struct Coords {
    pub lat: f32,
    pub lon: f32,
}

pub struct OpenWeatherClient<'a> {
    api_key: &'a str,
}

/// OpenWeather client instance
impl<'a> OpenWeatherClient<'a> {
    pub fn new(api_key: &'a str) -> Self {
        Self { api_key }
    }

    pub fn onecall_url(&self, coords: &Coords) -> String {
        format!(
            "https://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&appid={}",
            coords.lat, coords.lon, self.api_key
        )
    }

    pub fn onecall(&self, coords: &Coords) -> Result<OnecallResponse, reqwest::Error> {
        Ok(get(&self.onecall_url(coords))?.json::<OnecallResponse>()?)
    }
}

#[cfg(test)]
mod tests {
    use crate::OnecallResponse;

    #[test]
    fn deserialize_onecall_response() {
        let file = std::fs::File::open("./test/onecall_response.json").unwrap();
        let reader = std::io::BufReader::new(file);
        let _response: OnecallResponse = serde_json::from_reader(reader).unwrap();
    }
}
