use std::cmp::Ordering;

use crate::providers::{WeatherGetter, WeatherResult};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.weatherapi.com/v1";

#[derive(Debug, Clone)]
pub struct WeatherApi {
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentDetails {
    temp_c: f32,
    wind_kph: f32,
    humidity: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ForecastDay {
    avgtemp_c: f32,
    avghumidity: f32,
    maxwind_kph: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ForecastDays {
    day: ForecastDay,
}

#[derive(Debug, Serialize, Deserialize)]
struct ForecastDetails {
    forecastday: Vec<ForecastDays>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentAPIResponse {
    current: CurrentDetails,
}

#[derive(Debug, Serialize, Deserialize)]
struct ForecastAPIResponse {
    forecast: ForecastDetails,
}

impl WeatherApi {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    fn get_current_weather(&self, address: &str) -> WeatherResult {
        let url = format!("{API_URL}/current.json");
        let resp: CurrentAPIResponse = ureq::get(&url)
            .query("key", &self.api_key)
            .query("q", address)
            .call()?
            .into_json()?;
        Ok(format!(
            "Temperature: {}, humidity: {}, wind speed: {}",
            resp.current.temp_c, resp.current.humidity, resp.current.wind_kph
        ))
    }

    fn get_forecast_weather(&self, path: &str, address: &str, date: &str) -> WeatherResult {
        let url = format!("{API_URL}/{path}");
        let resp: ForecastAPIResponse = ureq::get(&url)
            .query("key", &self.api_key)
            .query("q", address)
            .query("dt", date)
            .call()?
            .into_json()?;

        match resp.forecast.forecastday.first() {
            Some(ForecastDays { day }) => Ok(format!(
                "Average temperature: {}, average humidity: {}, max wind speed: {}",
                day.avgtemp_c, day.avghumidity, day.maxwind_kph
            )),
            None => Ok("No data".to_string()),
        }
    }

    fn get_future_weather(&self, address: &str, date: &str) -> WeatherResult {
        self.get_forecast_weather("future.json", address, date)
    }

    fn get_historical_weather(&self, address: &str, date: &str) -> WeatherResult {
        self.get_forecast_weather("history.json", address, date)
    }
}

impl WeatherGetter for WeatherApi {
    fn get_weather(&self, address: &str, date: &str) -> WeatherResult {
        if date.is_empty() {
            self.get_current_weather(address)
        } else {
            let target = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
            let now = Local::now().date_naive();

            match target.cmp(&now) {
                Ordering::Equal => self.get_current_weather(address),
                Ordering::Greater => self.get_future_weather(address, &target.to_string()),
                Ordering::Less => self.get_historical_weather(address, &target.to_string()),
            }
        }
    }
}
