//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::Menu;

use gtk4::prelude::*;
use gtk4::{
    MenuButton,
    PopoverMenu
};


pub fn main_menu_new() -> MenuButton {
    let button = MenuButton::new();
    button.set_icon_name("open-menu-symbolic");
    button.set_tooltip_text(Some("Main Menu"));

    let menu = Menu::new();
    menu.append(Some("Preferences"), Some("app.preferences"));
    menu.append(Some("Keyboard Shortcuts"), Some("app.shortcuts"));
    menu.append(Some("About Bobby"), Some("app.about"));

    let popover_menu = PopoverMenu::from_model(Some(&menu));
    button.set_popover(Some(&popover_menu));

    button
}
