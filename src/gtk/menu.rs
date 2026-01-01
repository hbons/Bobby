//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::MenuButton;
use gtk4::gio::{ Menu, SimpleAction };
use libadwaita::Application;

use super::about::show_about_dialog;
use super::preferences::show_preferences_dialog;


pub fn main_menu_new(application: &Application) -> MenuButton {
    let preferences_action = SimpleAction::new("preferences", None);
    let application_handle = application.clone();

    preferences_action.connect_activate(move |_, _| {
        if let Some(active_window) = application_handle.active_window() {
            show_preferences_dialog(&active_window, None);
        }
    });


    let about_action = SimpleAction::new("about", None);
    let application_handle = application.clone();

    about_action.connect_activate(move |_, _| {
        if let Some(active_window) = application_handle.active_window() {
            show_about_dialog(&active_window);
        }
    });


    application.add_action(&preferences_action);
    application.add_action(&about_action);

    let menu = Menu::new();
    menu.append(Some("Preferences"), Some("app.preferences"));
    // menu.append(Some("Sponsors"), Some("app.sponsors")); // TODO
    menu.append(Some("About Bobby"), Some("app.about"));

    let button = MenuButton::new();
    button.set_icon_name("open-menu-symbolic");
    button.set_menu_model(Some(&menu));
    button.set_tooltip_text(Some("Main Menu"));
    button
}
