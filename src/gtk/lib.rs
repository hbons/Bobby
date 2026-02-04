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

use super::window::{
    window_empty_new,
    try_window_new,
};


impl Gui for App {
    // Docs: https://docs.gtk.org/gtk4
    //       https://gnome.pages.gitlab.gnome.org/libadwaita/doc

    fn gui_run(&self) -> Result<(), Box<dyn Error>> {
        let application = Application::builder()
            .application_id(&self.id)
            .flags(ApplicationFlags::HANDLES_OPEN)
            .build();

        application.connect_startup(|application| {
            application.add_action(&app_about_action(&application));
            application.add_action(&app_close_action(&application));
            application.add_action(&app_open_action(&application));
            application.add_action(&app_preferences_action(&application));
            application.add_action(&app_quit_action(&application));
            application.add_action(&app_shortcuts_action(&application));
        });

        application.connect_activate(|application| {
            if let Some(window) = application.active_window() {
                window.present();
            } else if let Ok(window) = window_empty_new(application) {
                window.present();
            }
        });

        application.connect_open(move |application, files, _| {
            for file in files {
                if let Some(path) = file.path() &&
                   let Some(window) = application.windows()
                       .iter()
                       .find(|w| w.widget_name().to_string() == path.to_string_lossy())
                {
                    window.present();
                } else {
                    try_window_new(application, file, true);
                }
            }
        });

        application.run();

        Ok(())
    }
}
