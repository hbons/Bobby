//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;
use gtk4::prelude::*;
use libadwaita::Application;

use crate::gtk::dialogs::preferences::show_preferences_dialog;


pub fn app_preferences_action(app: &Application) -> SimpleAction {
    app.set_accels_for_action("app.preferences", &["<Primary>comma"]);

    let action = SimpleAction::new("preferences", None);
    let app_handle = app.clone();

    action.connect_activate(move |_, _| {
        if let Some(active_window) = app_handle.active_window() {
            show_preferences_dialog(&active_window);
        }
    });

    action
}
