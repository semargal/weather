# Weather CLI (Rust Test Task)

The project provides users with a CLI tool to get weaher details for a given location and date. 

The CLI is written in Rust.

### Features

The Weather CLI supports these weather providers:
- [WeatherAPI](https://www.weatherapi.com)
- [ApiNinjas](https://api-ninjas.com)

Available data: temperature, humidity and wind speed.

### How To Use

1. Build and install the Weather CLI:

```shell
cargo install --git https://github.com/semargal/weather
```

2. Configure your weather provider:

The CLI requires `API_KEY` to interact with APIs. 

```shell
weather configure <PROVIDER> <API_KEY>
```
Example:
```shell
weather configure weather-api 319bbe911aaaa11111a222222222222
```

3. Use the following command to get weather data:

```shell
weather get <CITY> [date]
```
Example for current date:
```shell
weather get London
```
Output:
```shell
Temperature: 3, humidity: 81, wind speed: 6.1
```

Example example for forecasts:
```shell
weather get London 2023-04-03
```
Output:
```shell
Average temperature: 9.9, average humidity: 76, max wind speed: 17.3
```
