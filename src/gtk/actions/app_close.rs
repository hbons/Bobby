//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;
use gtk4::prelude::*;
use libadwaita::Application;


pub fn app_close_action(app: &Application) -> SimpleAction {
    app.set_accels_for_action("app.close", &["<Primary>w"]);

    let action = SimpleAction::new("close", None);
    let app_handle = app.clone();

    action.connect_activate(move |_, _| {
        if let Some(window) = app_handle.active_window() {
            // TODO: Remove database from memory here
            window.close();
        }
    });

    action
}
