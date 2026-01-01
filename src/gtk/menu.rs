//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::MenuButton;
// use gtk4::gio::Menu;
use libadwaita::Application;


pub fn main_menu_new(application: &Application) -> MenuButton {
    let button = MenuButton::new();
    button.set_icon_name("open-menu-symbolic");
    button.set_tooltip_text(Some("Main Menu"));

    let menu = unsafe {
        application.data::<gio::Menu>("menu")
            .map(|db| db.as_ref())

    };

    let popover_menu = gtk4::PopoverMenu::from_model(Some(menu.unwrap()));
    button.set_popover(Some(&popover_menu));

    button
}
