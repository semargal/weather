use std::error;

pub mod api_ninjas;
pub mod weaterh_api;

pub type WeatherResult = Result<String, Box<dyn error::Error>>;

pub trait WeatherGetter {
    fn get_weather(&self, address: &str, date: &str) -> WeatherResult;
}
