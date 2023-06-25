use gtk::prelude::*;
use gtk::{Align, Label, Window, WindowType, Box, Orientation, Image};
use std::collections::HashMap;
use std::fs;
use serde::{Deserialize};
use reqwest::blocking::get;

#[derive(Deserialize, Debug)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct WeatherData {
    main: HashMap<String, f32>,
    weather: Vec<Weather>,
}

pub fn run() {
    let api_key = fs::read_to_string("src/config.txt")
    .expect("Something went wrong reading the file");

    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Weather Forecast");
    window.set_default_size(400, 300);

    let vbox = Box::new(Orientation::Vertical, 10);

    let city_label = Label::new(None);
    city_label.set_text("London");

    let temp_label = Label::new(None);

    let image = Image::from_file("src/few_clouds.jpg"); // Replace this with the appropriate image based on the weather status

    vbox.pack_start(&city_label, true, true, 0);
    vbox.pack_start(&temp_label, true, true, 0);
    vbox.pack_start(&image, true, true, 0);

    window.add(&vbox);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    let response_body = get(format!("http://api.openweathermap.org/data/2.5/weather?q=London,uk&units=metric&APPID={}", api_key))
    .unwrap()
    .text()
    .unwrap();

    println!("Response body: {}", response_body);

    let response: WeatherData = serde_json::from_str(&response_body).unwrap();

    let weather_text = format!(
        "In London, it's currently {} degrees Celsius with {}.",
        response.main.get("temp").unwrap(),
        response.weather[0].description
    );

    temp_label.set_text(&weather_text);

    gtk::main();
}
