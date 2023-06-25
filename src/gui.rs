use gtk::prelude::*;
use gtk::{Label, Window, WindowType};
use std::collections::HashMap;
use std::fs;
use serde::{Deserialize};
use reqwest::blocking::get;

#[derive(Deserialize, Debug)]
struct WeatherData {
    main: HashMap<String, f32>,
    weather: Vec<HashMap<String, String>>,
}

pub fn run() {
    let api_key = fs::read_to_string("src/config.txt")
    .expect("Something went wrong reading the file");


    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Weather Forecast");
    window.set_default_size(200, 100);

    let label = Label::new(None);
    window.add(&label);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    let response_body = get(format!("http://api.openweathermap.org/data/2.5/weather?q=London,uk&APPID={}", api_key))
    .unwrap()
    .text()
    .unwrap();

println!("Response body: {}", response_body);

let response: WeatherData = serde_json::from_str(&response_body).unwrap();
    let weather = format!(
        "In London, it's currently {} degrees Celsius with {}.",
        response.main.get("temp").unwrap(),
        response.weather[0].get("description").unwrap()
    );

    glib::idle_add_local(move || {
        label.set_text(&weather);
        glib::Continue(false)
    });

    gtk::main();
}
