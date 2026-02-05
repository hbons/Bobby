//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;

use gtk4::prelude::*;
use gtk4::{
    glib::Variant,
    MenuButton,
};

use libadwaita::ApplicationWindow;

use crate::bobby::prelude::*;
use crate::gtk::windows::window::window_change_content;


pub fn switch_table_action(
    window: &ApplicationWindow,
    layout: gtk4::Box,
    table_index: String,
    tables: Vec<Table>,
    switcher: MenuButton,
) -> SimpleAction
{
    let action = SimpleAction::new_stateful(
        "table",
        Some(&String::static_variant_type()),
        &Variant::from(table_index),
    );

    let window_handle = window.clone();
    let layout_handle = layout.clone();
    let switcher_handle = switcher.clone();

    action.connect_change_state(move |action, value| {
        if let Some(v) = value {
            action.set_state(v);
        }

        if let Some(table) = value
            .and_then(|v| v.str())
            .and_then(|s| s.parse::<usize>().ok())
            .and_then(|i| tables.get(i))
        {
            switcher_handle.set_label(&table.name());

            match window_change_content(&window_handle, table) {
                Ok(new_content) => {
                    if let Some(old_content) = layout_handle.last_child() {
                        layout_handle.remove(&old_content);
                        layout_handle.append(&new_content);
                    }
                },
                Err(e) => eprintln!("Could not change content: {e}"),
            };
        }
    });

    action
}
