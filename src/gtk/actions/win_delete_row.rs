//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;

use gtk4::prelude::*;
use gtk4::{
    glib::{
        Variant,
        VariantTy,
    },
};

use libadwaita::{
    ApplicationWindow,
    Toast,
    ToastOverlay,
};

use crate::bobby::prelude::*;
use crate::gtk::windows::window::{
    find_column_view,
    get_row,
};


pub fn delete_row_action(
    window: &ApplicationWindow,
    overlay: &ToastOverlay,
) -> SimpleAction
{
    let action = SimpleAction::new("delete-row", Some(VariantTy::STRING));

    let window_handle = window.clone();
    let overlay_handle = overlay.clone();

    action.connect_activate(move |_, row_index| {
        let Some(column_view) = find_column_view(window_handle.upcast_ref()) else {
            return;
        };

        let Some(row_index) = row_index
            .and_then(|v| v.str())
            .and_then(|s| s.parse::<usize>().ok()) else {
            return;
        };

        let Some(row) = get_row(column_view, row_index) else {
            return;
        };

        let db = unsafe {
            window_handle
                .data::<Database>("db")
                .map(|db| db.as_ref().clone())
        };

        let Some(db) = db else {
            return;
        };

        let table_index = unsafe {
            window_handle
                .data::<String>("table-index")
                .and_then(|index| index.as_ref().parse::<usize>().ok())
        };

        let Some(table_index) = table_index else {
            return;
        };

        let table = db
            .tables()
            .ok()
            .and_then(|tables| tables.get(table_index).cloned());

        let Some(table) = table else {
            return;
        };

        match db.delete_row(&table, &row) {
            Ok(0) => {
                overlay_handle.dismiss_all();
                overlay_handle.add_toast(
                    Toast::builder()
                        .title("No row deleted")
                        .timeout(2)
                        .build()
                );
            },
            Ok(_) => {
                if let Some(action) = window_handle.lookup_action("table") {
                    action.change_state(
                        &Variant::from(table_index.to_string())
                    );
                }

                overlay_handle.dismiss_all();
                overlay_handle.add_toast(
                    Toast::builder()
                        .title(format!("Row {} deleted", row_index + 1))
                        .timeout(2)
                        .build()
                );
            },
            Err(e) => {
                overlay_handle.dismiss_all();
                overlay_handle.add_toast(
                    Toast::builder()
                        .title(format!("Could not delete row: {e}"))
                        .timeout(4)
                        .build()
                );
            },
        }
    });

    action
}
