//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::SimpleAction;
use gtk4::ColumnView;
use gtk4::glib::VariantTy;

use libadwaita::{
    ApplicationWindow,
    Toast,
    ToastOverlay,
};

use crate::gtk::widgets::content::get_row;
use crate::gtk::util::{
    copy_to_clipboard,
    widget_by_name,
};


pub fn copy_val_action(
    window: &ApplicationWindow,
    overlay: &ToastOverlay,
) -> SimpleAction
{
    let action = SimpleAction::new("copy-val", Some(VariantTy::STRING));

    let window_handle = window.clone();
    let overlay_handle = overlay.clone();

    action.connect_activate(move |_, row_col_index| {
        if let Some(column_view) = widget_by_name::<ColumnView>("column_view", &window_handle) {
            if let Some((row_index, col_index)) = row_col_index
                .and_then(|v| v.str())
                .and_then(|s| s.split_once(':'))
            {
                let row_index = row_index.parse::<usize>().unwrap_or_default();
                let col_index = col_index.parse::<usize>().unwrap_or_default();

                if let Some(row) = get_row(column_view, row_index) &&
                    let Some(cell) = row.cells.get(col_index) {
                    let selection = &cell.to_string();
                    let r = copy_to_clipboard(selection);

                    dbg!(&r);

                    let title = if selection.len() < 96 {
                        &format!("<span font_features='tnum=1'>‘{selection}’  copied to clipboard</span>")
                    } else {
                        "Copied to clipboard"
                    };

                    if selection.len() < 96 {
                        overlay_handle.dismiss_all();
                        overlay_handle.add_toast(
                            Toast::builder()
                                .title(title)
                                .timeout(2)
                                .build()
                        );
                    }
                }
            }
        }
    });

    action
}
