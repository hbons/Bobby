//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;
use gtk4::prelude::*;
use libadwaita::Application;

use crate::bobby::prelude::*;
use crate::gtk::windows::window::window_new;
use crate::gtk::windows::window_empty::IS_EMPTY_WINDOW;


pub fn reload_action(app: &Application) -> SimpleAction {
    app.set_accels_for_action("app.reload", &["<Primary>r"]);

    let action = SimpleAction::new("reload", None);
    let app_handle = app.clone();

    action.connect_activate(move |_, _| {
        if let Some(active_window) = app_handle.active_window() {
            if active_window.widget_name() == IS_EMPTY_WINDOW {
                return;
            }

            let db = unsafe {
                active_window
                    .data::<Database>("db")
                    .map(|db| db.as_ref())
            };

            active_window.destroy();
            let quit_on_close = false;

            if let Some(db) = db.as_ref() &&
               let Ok(window) = window_new(&app_handle, db, None, quit_on_close) {
                // TODO: Scroll to previous row number
                window.present();
            }
        }
    });

    action
}
