//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;
use gtk4::prelude::*;
use libadwaita::Application;

use crate::gtk::windows::window::window_new;


pub fn new_action(app: &Application) -> SimpleAction {
    app.set_accels_for_action("app.new", &["<Primary>n"]);

    let action = SimpleAction::new("new", None);
    let app_handle = app.clone();

    action.connect_activate(move |_, _| {
        if let Ok(w) = window_new(&app_handle, None, None) {
            w.present();
        }
    });

    action
}
