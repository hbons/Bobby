//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;
use gtk4::prelude::*;
use libadwaita::Application;

use crate::gtk::dialogs::shortcuts::show_shortcuts_dialog;


pub fn app_shortcuts_action(app: &Application) -> SimpleAction {
    app.set_accels_for_action("app.shortcuts", &["<Primary>question"]);

    let action = SimpleAction::new("shortcuts", None);
    let app_handle = app.clone();

    action.connect_activate(move |_, _| {
        if let Some(active_window) = app_handle.active_window() {
            show_shortcuts_dialog(&active_window);
        }
    });

    action
}
