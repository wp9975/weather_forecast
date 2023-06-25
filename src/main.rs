use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Hello, World!");

    let button = Button::with_label("Click me!");
    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}