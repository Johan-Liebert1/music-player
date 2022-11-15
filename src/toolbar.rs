use crate::gtk::prelude::{
    ApplicationExt, ApplicationExtManual, ContainerExt, GtkWindowExt, WidgetExt,
};

use gtk::{SeparatorToolItem, ToolButton, Toolbar};

pub const PLAY_STOCK: &str = "gtk-media-play";
pub const PAUSE_STOCK: &str = "gtk-media-pause";

pub struct MusicToolbar {
    pub open_button: ToolButton,
    pub next_button: ToolButton,
    pub play_button: ToolButton,
    pub previous_button: ToolButton,
    pub quit_button: ToolButton,
    pub remove_button: ToolButton,
    pub stop_button: ToolButton,
    pub toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();
        // window.add(&toolbar);

        let open_button = ToolButton::new(ToolButton::NONE, Some("gtk-open"));
        toolbar.add(&open_button);
        toolbar.add(&SeparatorToolItem::new());

        let previous_button = ToolButton::new(ToolButton::NONE, Some("gtk-media-previous"));
        toolbar.add(&previous_button);

        let play_button = ToolButton::new(ToolButton::NONE, Some(PLAY_STOCK));
        toolbar.add(&play_button);

        let stop_button = ToolButton::new(ToolButton::NONE, Some("gtk-media-stop"));
        toolbar.add(&stop_button);

        let next_button = ToolButton::new(ToolButton::NONE, Some("gtk-media-next"));
        toolbar.add(&next_button);
        toolbar.add(&SeparatorToolItem::new());

        let remove_button = ToolButton::new(ToolButton::NONE, Some("gtk-remove"));
        toolbar.add(&remove_button);
        toolbar.add(&SeparatorToolItem::new());

        let quit_button = ToolButton::new(ToolButton::NONE, Some("gtk-quit"));
        toolbar.add(&quit_button);

        MusicToolbar {
            open_button,
            next_button,
            play_button,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
        }
    }

    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}
