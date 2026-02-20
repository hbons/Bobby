//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::{
    Menu,
    SimpleAction,
};

use gtk4::prelude::*;
use gtk4::{
    MenuButton,
    PopoverMenu
};

use libadwaita::Application;


pub fn main_menu_new(app: &Application) -> MenuButton {
    let button = MenuButton::new();
    button.set_icon_name("open-menu-symbolic");
    button.set_tooltip_text(Some("Main Menu"));

    let button_clone = button.clone();

    let action = SimpleAction::new("open-menu", None);

    action.connect_activate(move |_, _| {
        button_clone.activate();
    });

    app.add_action(&action);
    app.set_accels_for_action("app.open-menu", &["F10"]);

    let menu = Menu::new();
    menu.append(Some("Preferences"), Some("app.preferences"));
    menu.append(Some("Keyboard Shortcuts"), Some("app.shortcuts"));
    menu.append(Some("About Bobby"), Some("app.about"));

    button.set_popover(
        Some(&PopoverMenu::from_model(Some(&menu)))
    );

    button
}
