//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;
use gtk4::prelude::*;
use libadwaita::Application;


pub fn quit_action(app: &Application) -> SimpleAction {
    app.set_accels_for_action("app.quit", &["<Primary>q"]);

    let action = SimpleAction::new("quit", None);
    let app_handle = app.clone();

    action.connect_activate(move |_, _| {
        app_handle.quit();
    });

    action
}
