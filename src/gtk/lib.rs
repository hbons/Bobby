//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gio::ApplicationFlags;
use gtk4::prelude::*;
use libadwaita::Application;

use crate::app::App;
use crate::gtk::actions::prelude::*;
use crate::gui::Gui;

use crate::gtk::windows::window_empty::window_empty_new;
use crate::gtk::windows::window::try_window_new;


impl Gui for App {
    // Docs: https://docs.gtk.org/gtk4
    //       https://gnome.pages.gitlab.gnome.org/libadwaita/doc

    fn gui_run(&self) -> Result<(), Box<dyn Error>> {
        let app = Application::builder()
            .application_id(&self.id)
            .flags(ApplicationFlags::HANDLES_OPEN)
            .build();

        app.connect_activate(|app| {
            if let Some(window) = app.active_window() {
                window.present();
            } else if let Ok(window) = window_empty_new(app) {
                window.present();
            }
        });

        app.connect_open(move |app, files, _| {
            for file in files {
                if let Some(path) = file.path() &&
                   let Some(window) = app.windows()
                       .iter()
                       .find(|w| w.widget_name().to_string() == path.to_string_lossy())
                {
                    window.present();
                } else {
                    try_window_new(app, file, true);
                }
            }
        });

        app.add_action(&app_about_action(&app));
        app.add_action(&app_close_action(&app));
        app.add_action(&app_open_action(&app));
        app.add_action(&app_preferences_action(&app));
        app.add_action(&app_quit_action(&app));
        app.add_action(&app_shortcuts_action(&app));

        app.run();

        Ok(())
    }
}
