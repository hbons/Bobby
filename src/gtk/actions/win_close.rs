//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;
use gtk4::prelude::*;
use libadwaita::ApplicationWindow;


pub fn close_action(window: &ApplicationWindow) -> SimpleAction {
    if let Some(app) = window.application() {
        app.set_accels_for_action("win.close", &["<Primary>w"]);
    }

    let action = SimpleAction::new("close", None);
    let window_handle = window.clone();

    action.connect_activate(move |_, _| {
        // TODO: Remove database from memory here
        window_handle.close();
    });

    action
}
