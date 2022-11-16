extern crate gio;
extern crate gtk;
extern crate gtk_sys;

extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

mod mp3;
mod player;
mod playlist;
mod toolbar;

use std::{env, rc::Rc};

use crate::gtk::prelude::{
    ApplicationExt, ApplicationExtManual, ContainerExt, DialogExt, FileChooserExt, GtkWindowExt,
    ImageExt, ScaleExt, ToolButtonExt, WidgetExt, WidgetExtManual,
};
use gtk::{
    Adjustment, Application, ApplicationWindow, Image,
    Orientation::{Horizontal, Vertical},
    Scale,
};

use playlist::Playlist;
use toolbar::{MusicToolbar, PAUSE_STOCK, PLAY_STOCK};

use gio::ApplicationFlags;

use gtk::{FileChooserAction, FileChooserDialog, FileFilter};
use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};
use std::path::PathBuf;

const RESPONSE_ACCEPT: i32 = GTK_RESPONSE_ACCEPT as i32;
const RESPONSE_CANCEL: i32 = GTK_RESPONSE_CANCEL as i32;

fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;

    let dialog = FileChooserDialog::new(
        Some("Select an MP3 audio file"),
        Some(parent),
        FileChooserAction::Open,
    );

    let filter = FileFilter::new();
    filter.add_mime_type("audio/mp3");
    filter.set_name(Some("MP3 audio file"));

    dialog.add_filter(&filter);
    dialog.add_button("Cancel", gtk::ResponseType::Cancel);
    dialog.add_button("Accept", gtk::ResponseType::Accept);

    let result = dialog.run();

    println!("result of dialog.run() {:?}", result);

    if result == gtk::ResponseType::Accept {
        file = dialog.filename();
    }

    unsafe {
        dialog.destroy();
    }

    file
}

struct App {
    adjustment: Adjustment,
    cover: Image,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
    playlist: Rc<Playlist>,
}

impl App {
    fn new(application: &Application) -> Self {
        let window = ApplicationWindow::new(application);
        window.set_title("Music");

        let vbox = gtk::Box::new(Vertical, 0);
        window.add(&vbox);

        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());

        let playlist = Rc::new(Playlist::new());
        vbox.add(playlist.view());

        let cover = Image::new();
        // cover.set_from_file(Some("/media/pragyan/Local Disk/Dark_Side_of_the_Moon.png"));
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
            playlist,
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
        let playlist = self.playlist.clone();
        let cover = self.cover.clone();

        self.toolbar.play_button.connect_clicked(move |_| {
            if let Some(current_label) = play_button.label() {
                if current_label == PLAY_STOCK {
                    play_button.set_label(Some(PAUSE_STOCK));
                    toolbar::set_cover(&cover, &playlist);
                } else {
                    play_button.set_label(Some(PLAY_STOCK));
                }
            }
        });

        let parent = self.window.clone();
        let playlist = self.playlist.clone();

        self.toolbar.open_button.connect_clicked(move |_| {
            let file = show_open_dialog(&parent);

            println!("Option file {:?}", file);

            if let Some(file) = file {
                println!("{:?}", file);
                playlist.add(&file);
            }
        });

        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked(move |_| {
            playlist.remove_selection();
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
