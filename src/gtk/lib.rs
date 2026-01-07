//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::error::Error;

use gio::{
    Menu,
    SimpleAction,
};

use gtk4::prelude::*;
use gtk4::gio::ApplicationFlags;

use libadwaita::Application;

use crate::app::App;
use crate::bobby::sqlite::database::Database;
use crate::gui::Gui;

use super::window::{
    window_empty_new,
    window_new,
};

use super::preferences::show_preferences_dialog;
use super::about::show_about_dialog;


impl Gui for App {
    // Docs: https://docs.gtk.org/gtk4/

    fn gui_run(&self) -> Result<(), Box<dyn Error>> {
        let application = Application::builder()
            .application_id(&self.id)
            .flags(ApplicationFlags::HANDLES_OPEN)
            .build();

        application.set_accels_for_action("app.preferences", &["<Control>comma"]);


        application.connect_startup(|application| {
            let preferences_action = SimpleAction::new("preferences", None);
            let application_handle = application.clone();

            preferences_action.connect_activate(move |_, _| {
                if let Some(active_window) = application_handle.active_window() {
                    show_preferences_dialog(&active_window);
                }
            });


            let about_action = SimpleAction::new("about", None);
            let application_handle = application.clone();

            about_action.connect_activate(move |_, _| {
                if let Some(active_window) = application_handle.active_window() {
                    show_about_dialog(&active_window);
                }
            });


            let action = SimpleAction::new("sponsors", None);
            let application_handle = application.clone();

            action.connect_activate(move |_, _| {
                if let Some(active_window) = application_handle.active_window() {
                    show_about_dialog(&active_window);
                }
            });


            application.add_action(&preferences_action);
            application.add_action(&action);
            application.add_action(&about_action);


            let menu = Menu::new();
            menu.append(Some("Preferences"), Some("app.preferences"));
            // menu.append(Some("Sponsors"), Some("app.sponsors")); // TODO
            menu.append(Some("About Bobby"), Some("app.about"));

            unsafe { application.set_data("menu", menu); }
        });


        application.connect_activate(|application| {
            if let Some(window) = application.active_window() {
                window.present();
            } else {
                // TODO: Sometimes still opens a duplicate window if 1st was opened by "empty" state
                if let Ok(window) = window_empty_new(application) {
                    window.present();
                }
            }
        });


        application.connect_open(move |application, files, _| {
            if let Some(path) = files.first().and_then(|f| f.path()) {
                for window in application.windows() {
                    // SAFETY: Window outlives the database
                    let db = unsafe {
                        window
                            .data::<Database>("db")
                            .map(|db| db.as_ref())
                    };

                    if let Some(db) = db {
                        if db.path == path {
                            window.present();
                            return;
                        }
                    }
                }

                let table_name = env::args().nth(2);
                let result = window_new(application, &path, table_name);

                if let Ok(window) = result {
                    window.present();
                }
            }
        });


        application.run();
        Ok(())
    }
}
