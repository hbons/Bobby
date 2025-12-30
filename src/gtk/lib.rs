//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::error::Error;

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


impl Gui for App {
    // Docs: https://docs.gtk.org/gtk4/

    fn gui_run(&self) -> Result<(), Box<dyn Error>> {
        let application = Application::builder()
            .application_id(&self.id)
            .flags(ApplicationFlags::HANDLES_OPEN)
            .build();

        application.set_accels_for_action("app.preferences", &["<Control>comma"]);

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
