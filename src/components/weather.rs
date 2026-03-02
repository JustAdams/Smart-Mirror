use serde::{Deserialize, Serialize};
use slint::Image;
use std::env;
use std::io::Read;
use std::path::Path;

pub struct CurrentWeatherInfo {
    pub temp: f32,
    pub feels_like: f32,
    pub image: Image,
}
impl CurrentWeatherInfo {
    pub fn default() -> Self {
        Self {
            feels_like: 73.3,
            temp: 75.9,
            image: Image::load_from_path(Path::new("assets/weather-icons/sunny.png"))
                .expect("Failed to load image"),
        }
    }
    pub fn update_weather(&mut self) -> () {
        let mut current_weather: WeatherApiResponse = WeatherApiResponse::default();
        current_weather.build_weather();
        let weather_type: WeatherType = get_weather_type(&current_weather);
        self.feels_like = current_weather.feels_like;
        self.temp = current_weather.temp;
        self.image = determine_weather_image(weather_type);
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    current: WeatherApiResponse,
}
#[derive(Deserialize, Debug, Serialize)]
struct WeatherApiResponse {
    clouds: i32,
    feels_like: f32,
    temp: f32,
}
impl WeatherApiResponse {
    fn default() -> Self {
        Self {
            clouds: 50,
            feels_like: 73.3,
            temp: 75.9,
        }
    }
    fn build_weather(&mut self) -> () {
        let weather_url: String = env::var("WEATHER_URL").expect("WEATHER_URL missing from .env");

        let mut weather_response = reqwest::blocking::get(weather_url).unwrap();
        let mut body = String::new();
        weather_response.read_to_string(&mut body).unwrap();

        let api_response: ApiResponse = serde_json::from_str(&*body).unwrap();
        let current_weather: WeatherApiResponse = api_response.current;

        println!("curr temp: {0}", current_weather.temp);

        self.temp = current_weather.temp;
        self.clouds = current_weather.clouds;
        self.feels_like = current_weather.feels_like;
    }
}

enum WeatherType {
    Sunny,
    SunnyCloudy,
    Rainy,
    Cloudy,
}

fn get_weather_type(weather: &WeatherApiResponse) -> WeatherType {
    // todo: improve weather logic stuff
    let clouds = weather.clouds;
    if clouds >= 75 {
        WeatherType::Cloudy
    } else if clouds >= 49 {
        WeatherType::SunnyCloudy
    } else {
        WeatherType::Sunny
    }
}

fn determine_weather_image(weather_type: WeatherType) -> Image {
    let weather_icon_path = match weather_type {
        WeatherType::Sunny => "assets/weather-icons/sunny.png",
        WeatherType::Rainy => "assets/weather-icons/rainy.png",
        WeatherType::Cloudy => "assets/weather-icons/cloudy.png",
        WeatherType::SunnyCloudy => "assets/weather-icons/sunny_cloudy.png",
    };

    Image::load_from_path(Path::new(weather_icon_path)).expect("Failed to load image")
}
