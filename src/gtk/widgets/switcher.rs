//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gettextrs::{ gettext, ngettext };

use gtk4::prelude::*;
use gtk4::gio::Menu;
use gtk4::MenuButton;

use crate::bobby::prelude::*;


const WIDGET_NAME: &str = "switcher";

pub fn table_switcher_new(tables: &Vec<Table>) -> MenuButton {
    let menu = Menu::new();
    let button = MenuButton::builder()
        .menu_model(&menu)
        .build();

    let table_section = Menu::new();
    let view_section  = Menu::new();

    let view_count  = tables.iter().filter(|t|  t.is_view()).count() as u32;
    let table_count = tables.iter().filter(|t| !t.is_view()).count() as u32;

    for (i, table) in tables.iter().enumerate() {
        let name = table.name().replace("_", "__"); // Avoid mnemonics

        let section = if table.is_view() {
            &view_section
        } else {
            &table_section
        };

        section.append(
            Some(&name),
            Some(&format!("win.table::{}", i)),
        );
    }

    let views_label =
        ngettext("View – {n}", "Views – {n}", view_count)
            .replace("{n}", &view_count.to_string());
    let tables_label =
        ngettext("Table – {n}", "Tables – {n}", table_count)
            .replace("{n}", &table_count.to_string());

    menu.append_section(Some(&views_label),  &view_section);
    menu.append_section(Some(&tables_label), &table_section);

    if let Some(table) = tables.first() {
        button.set_label(&table.name());
    }

    // button.set_action_name(Some("win.open-table-menu")); // TODO

    button.set_widget_name(WIDGET_NAME);
    button.set_menu_model(Some(&menu));
    button.set_tooltip_text(Some(&gettext("Tables")));
    button
}
