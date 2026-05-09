//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;
use gtk4::prelude::*;
use libadwaita::ApplicationWindow;

use crate::bobby::prelude::*;
use crate::gtk::windows::window::IS_EMPTY_WINDOW;
use crate::gtk::windows::window::window_reload;


pub fn reload_action(window: &ApplicationWindow) -> SimpleAction {
    if let Some(app) = window.application() {
        app.set_accels_for_action("win.reload", &["<Primary>r"]);
    }

    let action = SimpleAction::new("reload", None);
    let window_handle = window.clone();

    action.connect_activate(move |_, _| {
        // TODO: Remove database from memory here

        if window_handle.widget_name() == IS_EMPTY_WINDOW {
            return;
        }

        let db = unsafe {
            window_handle
                .data::<Database>("db")
                .map(|db| db.as_ref())
        };

        if let Some(app) = window_handle.application() &&
           let Ok(app) = app.downcast::<libadwaita::Application>() &&
           let Some(db) = db.as_ref() {
            _ = window_reload(&app, &db.file);
        }
    });

    action
}
