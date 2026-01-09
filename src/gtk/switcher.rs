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

    let table_section = Menu::new();
    let view_section  = Menu::new();

    let view_count  = tables.iter().filter(|t| t.is_view()).count();
    let table_count = tables.iter().filter(|t| !t.is_view()).count();

    for (i, table) in tables.iter().enumerate() {
        if table.is_view() {
            view_section.append(
                Some(&table.name()),
                Some(&format!("win.table::{}", i)),
            );
        } else {
            table_section.append(
                Some(&table.name()),
                Some(&format!("win.table::{}", i)),
            );
        }
    }

    menu.append_section(Some(&format!("Views – {view_count}")), &view_section);
    menu.append_section(Some(&format!("Tables – {table_count}")), &table_section);

    if let Some(table) = tables.first() {
        button.set_label(&table.name());
    }

    button.set_menu_model(Some(&menu));
    button.set_tooltip_text(Some("Select Table"));
    button
}
