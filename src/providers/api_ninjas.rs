use crate::providers::{WeatherGetter, WeatherResult};
use serde::{Deserialize, Serialize};
use ureq;

const API_URL: &str = "https://api.api-ninjas.com/v1/weather";

#[derive(Debug, Clone)]
pub struct ApiNinjas {
    api_key: String,
}

impl ApiNinjas {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct APIResponse {
    temp: f32,
    humidity: f32,
    wind_speed: f32,
}

impl WeatherGetter for ApiNinjas {
    fn get_weather(&self, address: &str, _: &str) -> WeatherResult {
        let resp: APIResponse = ureq::get(API_URL)
            .query("city", address)
            .set("X-Api-Key", &self.api_key)
            .call()?
            .into_json()?;

        let resp = format!(
            "Temperature: {}, humidity: {}, wind speed: {}",
            resp.temp, resp.humidity, resp.wind_speed
        );
        Ok(resp)
    }
}
