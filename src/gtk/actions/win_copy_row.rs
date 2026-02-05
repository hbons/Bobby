//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;

use gtk4::prelude::*;
use gtk4::{
    glib::VariantTy,
};

use libadwaita::{
    ApplicationWindow,
    Toast,
    ToastOverlay,
};

use crate::bobby::prelude::*;
use crate::gtk::windows::window::{
    copy_to_clipboard,
    find_column_view,
    get_row,
};


pub fn copy_row_action(
    window: &ApplicationWindow,
    overlay: &ToastOverlay,
) -> SimpleAction
{
    let action = SimpleAction::new("copy-row", Some(VariantTy::STRING));

    let window_handle = window.clone();
    let overlay_handle = overlay.clone();

    action.connect_activate(move |_, row_index| {
        if let Some(column_view) = find_column_view(window_handle.upcast_ref()) {
            if let Some(row_index) = row_index
                .and_then(|v| v.str())
                .and_then(|s| s.parse::<usize>().ok())
            {
                if let Some(row) = get_row(column_view, row_index) {
                    let settings = gio::Settings::new("studio.planetpeanut.Bobby"); // TODO
                    let separator = settings.string("column-separator");
                    let separator = separator.as_str().parse::<ColumnSeparator>();

                    _ = copy_to_clipboard(
                        &row.format_with(separator.unwrap_or_default())
                    );

                    overlay_handle.dismiss_all();
                    overlay_handle.add_toast(
                        Toast::builder()
                            .title(format!("Row {} copied to clipboard", row_index + 1))
                            .timeout(2)
                            .build()
                    );
                }
            }
        }
    });

    action
}
