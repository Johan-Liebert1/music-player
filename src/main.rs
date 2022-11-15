extern crate gio;
extern crate gtk;

extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

mod mp3;
mod player;
mod toolbar;

use std::env;

use crate::gtk::prelude::{
    ApplicationExt, ApplicationExtManual, ContainerExt, GtkWindowExt, ImageExt, ScaleExt,
    ToolButtonExt, WidgetExt, WidgetExtManual,
};
use gtk::{
    Adjustment, Application, ApplicationWindow, Image,
    Orientation::{Horizontal, Vertical},
    Scale,
};

use toolbar::{MusicToolbar, PAUSE_STOCK, PLAY_STOCK};

use gio::ApplicationFlags;

struct App {
    adjustment: Adjustment,
    cover: Image,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    fn new(application: &Application) -> Self {
        let window = ApplicationWindow::new(application);
        window.set_title("Music");

        let vbox = gtk::Box::new(Vertical, 0);
        window.add(&vbox);

        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());

        let cover = Image::new();
        cover.set_from_file(Some("/media/pragyan/Local Disk/Dark_Side_of_the_Moon.png"));
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, Some(&adjustment));
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.show_all();

        let app = App {
            toolbar,
            window,
            adjustment,
            cover,
        };

        app.connect_events(&application);
        app.connect_toolbar_events();

        app
    }

    fn connect_events(&self, application: &Application) {}

    pub fn connect_toolbar_events(&self) {
        let window = self.window.clone();

        self.toolbar.quit_button.connect_clicked(move |_| unsafe {
            window.destroy();
        });

        let play_button = self.toolbar.play_button.clone();

        self.toolbar.play_button.connect_clicked(move |_| {
            // if play_button.label() == Some(PLAY_STOCK) {
            if let Some(_) = play_button.label() {
                play_button.set_label(Some(PAUSE_STOCK));
            } else {
                play_button.set_label(Some(PLAY_STOCK));
            }
        });
    }
}

fn main() {
    let application = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    application.connect_startup(|app| {
        let application_struct = App::new(app);
        // let window = ApplicationWindow::new(app);
        // window.set_title("Music");
        // window.show_all();
    });

    application.connect_activate(|_| {});

    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);

    application.run();
}
