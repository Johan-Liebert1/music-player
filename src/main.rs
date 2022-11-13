extern crate gio;
extern crate gtk;

extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

mod mp3;
mod player;

use std::env;

use crate::gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt, WidgetExt};

use gio::ApplicationFlags;
use gtk::{Application, ApplicationWindow};

fn main() {
    // let application = Application::new(Some("Music Player"), ApplicationFlags::empty());

    let application = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    application.connect_startup(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Music");
        window.show_all();
    });

    application.connect_activate(|_| {});
    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);

    application.run();
}
