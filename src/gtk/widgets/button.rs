//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::{
    Align,
    Button,
};

use libadwaita::ApplicationWindow;


pub fn button_open_new(window: &ApplicationWindow) -> Button {
    let button = Button::builder()
        .label("Open...")
        .css_classes(["pill", "suggested-action"])
        .halign(Align::Center)
        .build();

    let window_weak = window.downgrade();

    button.connect_clicked(move |_| {
        if let Some(window) = window_weak.upgrade() {
            if let Some(application) = window.application() {
                application.activate_action("open", None);
            }
        }
    });

    button
}
