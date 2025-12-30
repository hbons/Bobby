//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::gio::Menu;
use gtk4::MenuButton;

use crate::bobby::prelude::*;


pub fn table_switcher_new(tables: &Vec<Table>) -> MenuButton {
    let menu = Menu::new();
    let button = MenuButton::builder()
        .menu_model(&menu)
        .build();

    let section = Menu::new();

    for table in tables {
        section.append(
            Some(table.name()),
            Some(&format!("win.table::{}", table.name())),
        );
    }

    menu.append_section(Some(&format!("Tables – {}", tables.len())), &section);
    // menu.append_section(Some("Views"), &section); // TODO

    if let Some(table) = tables.first() {
        button.set_label(table.name());
    }

    button.set_menu_model(Some(&menu));
    button.set_tooltip_text(Some(&"Select Table"));
    button
}
