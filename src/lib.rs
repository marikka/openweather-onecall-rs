//! Crate implementing the OpenWeather One Call API

use chrono::{serde::ts_seconds, DateTime, Utc};
use reqwest::blocking::get;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct OnecallResponse {
    pub lat: f32,
    pub lon: f32,
    pub timezone: String,
    pub timezone_offset: i32,
    pub current: Current,
    pub minutely: Option<Vec<Minutely>>,
    pub hourly: Vec<Hourly>,
    pub daily: Vec<Daily>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rain {
    #[serde(rename = "1h")]
    pub one_hour: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Current {
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub sunrise: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub sunset: DateTime<Utc>,
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: u32,
    pub humidity: u32,
    pub dew_point: f32,
    pub uvi: f32,
    pub clouds: u32,
    pub visibility: u32,
    pub wind_speed: f32,
    pub wind_deg: u32,
    pub wind_gust: Option<f32>,
    pub weather: Vec<Weather>,
    pub rain: Option<Rain>,
    pub alerts: Option<Vec<Alert>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Minutely {
    #[serde(with = "ts_seconds")]
    dt: DateTime<Utc>,
    precipitation: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Hourly {
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: u32,
    pub humidity: u32,
    pub dew_point: f32,
    pub uvi: f32,
    pub clouds: u32,
    pub visibility: u32,
    pub wind_speed: f32,
    pub wind_deg: u32,
    pub wind_gust: f32,
    pub weather: Vec<Weather>,
    pub pop: f32,
    pub rain: Option<Rain>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Temp {
    pub day: f32,
    pub min: f32,
    pub max: f32,
    pub night: f32,
    pub eve: f32,
    pub morn: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FeelsLike {
    pub day: f32,
    pub night: f32,
    pub eve: f32,
    pub morn: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Daily {
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub sunrise: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub sunset: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub moonrise: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub moonset: DateTime<Utc>,
    pub moon_phase: f32,
    pub temp: Temp,
    pub feels_like: FeelsLike,
    pub pressure: u32,
    pub humidity: u32,
    pub dew_point: f32,
    pub wind_speed: f32,
    pub wind_deg: u32,
    pub wind_gust: f32,
    pub weather: Vec<Weather>,
    pub clouds: u32,
    pub pop: f32,
    pub rain: Option<f32>,
    pub uvi: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Alert {
    pub sender_name: String,
    pub event: String,
    #[serde(with = "ts_seconds")]
    pub start: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub end: DateTime<Utc>,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
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
