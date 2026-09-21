//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;

use gtk4::prelude::*;
use gtk4::Window;

use libadwaita::ApplicationWindow;

use super::super::windows::window::{
    IS_EMPTY_WINDOW,
    window_toggle_search,
};


pub fn search_toggle_action(
    window: &ApplicationWindow,
) -> SimpleAction
{
    if let Some(app) = window.application() {
        app.set_accels_for_action(
            "win.search_toggle",
            &["<Primary>f"],
        );
    }

    let action = SimpleAction::new("search_toggle", None);
    let window_handle = window.clone();

    action.connect_activate(move |_, _| {
        if window_handle.widget_name() == IS_EMPTY_WINDOW {
            return;
        }

        let window = window_handle
            .clone()
            .upcast::<Window>();

        _ = window_toggle_search(&window);
    });

    action
}
