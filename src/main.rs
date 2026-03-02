mod components;

use chrono::Local;
use dotenv::dotenv;
use slint::{Image, SharedString, Weak};
use std::time::Duration;

use crate::components::quote;
use crate::components::weather::CurrentWeatherInfo;

slint::include_modules!();

fn main() {
    dotenv().ok();

    let main_window: MainWindow = MainWindow::new().unwrap();

    // update the time every second
    let ui_timer_handle = main_window.as_weak();
    let time_timer = slint::Timer::default();
    time_timer.start(
        slint::TimerMode::Repeated,
        Duration::from_secs(1),
        move || {
            if let Some(ui) = ui_timer_handle.upgrade() {
                let time_str = Local::now().format("%H:%M").to_string();
                ui.set_time_text(SharedString::from(time_str));
            }
        },
    );

    // weather shouldn't update very frequently to preserve API request limits
    let weather_timer = slint::Timer::default();
    let ui_weather_handle = main_window.as_weak();
    get_weather(&ui_weather_handle);

    weather_timer.start(
        slint::TimerMode::Repeated,
        Duration::from_secs(90),
        move || {
            get_weather(&ui_weather_handle);
        },
    );

    // daily quote
    let quote_timer = slint::Timer::default();
    let quote_handle = main_window.as_weak();

    quote_timer.start(
        slint::TimerMode::Repeated,
        Duration::from_secs(1),
        move || {
            if let Some(quote_ui) = quote_handle.upgrade() {
                let quote_str = quote::get_random_quote();
                quote_ui.set_quote_text(SharedString::from(quote_str));
            }
        },
    );

    main_window.run().unwrap();
}

fn get_weather(ui_weather_handle: &Weak<MainWindow>) {
    let mut weather_ui = CurrentWeatherInfo::default();
    weather_ui.update_weather();

    let actual_temp: String = weather_ui.temp.round().to_string();
    let feels_like_temp: String = weather_ui.feels_like.round().to_string();
    let weather_image: Image = weather_ui.image;

    if let Some(ui) = ui_weather_handle.upgrade() {
        ui.set_weather_image(weather_image);
        ui.set_temp_text(SharedString::from(actual_temp));
        ui.set_feels_like_text(SharedString::from(feels_like_temp));
    }
}
