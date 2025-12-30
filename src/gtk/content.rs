//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gtk4::prelude::*;
use gtk4::{
    Align,
    ColumnView,
    ColumnViewColumn,
    GestureClick,
    Label,
    ListItem,
    PickFlags,
    PopoverMenu,
    ScrolledWindow,
    SignalListItemFactory,
    SingleSelection,
};
use gtk4::gdk::{ BUTTON_SECONDARY, Rectangle };
use gtk4::gio::{ ListStore, Menu };
use gtk4::glib::Object;
use gtk4::pango::EllipsizeMode;

use crate::bobby::prelude::*;


// U+25C7 "White Diamond"
const SYMBOL_PRIMARY_KEY: &str = "◇";


pub fn content_new(columns: &Vec<Column>, rows: &Vec<Row>) -> ScrolledWindow {
    let store = ListStore::new::<Row>();

    for row in rows.iter().take(100_000) {
        store.append(row); // TODO: Remove hard limit when we have lazy loading
    }

    let selection = SingleSelection::new(Some(store));
    let column_view = ColumnView::new(Some(selection));

    for (i, column) in columns.iter().enumerate() {
        let factory = SignalListItemFactory::new();
        let affinity = column.affinity;

        factory.connect_setup(move |_, obj| {
            if let Err(e) = setup_list_item(obj) {
                eprintln!("Failed to set up list item: {e}");
            }
        });

        // TODO: Move out
        factory.connect_bind(move |_, obj| {
            if let Some(list_item) = obj.downcast_ref::<ListItem>() {
                if let Some(row) = list_item.item().and_downcast::<Row>() {
                    let cells = row.cells();
                    let text = cells.get(i)
                        .map(String::as_str)
                        .unwrap_or_default();

                    if let Some(label) = list_item.child().and_downcast::<Label>() {
                        // Reset state
                        label.set_sensitive(true);
                        label.set_tooltip_text(None);
                        // label.set_css_classes(&[]); // See below

                        // New state
                        label.set_text(text);
                        label.set_tooltip_text(Some(text));
                        label.set_sensitive(text != "NULL");

                        if affinity == Affinity::BLOB {
                            label.set_sensitive(false);
                        }

                        // if affinity != Affinity::TEXT { // TODO: Decide on this
                        //     label.set_css_classes(&["monospace"]);
                        // }
                    }
                }
            }
        });


        let view_column = ColumnViewColumn::new(
            Some(&column.name),
            Some(factory)
        );

        view_column.set_resizable(true);
        view_column.set_fixed_width(
            match column.affinity {
                Affinity::BLOB => { view_column.set_resizable(false); 128 },
                Affinity::TEXT => 192,
                _ => 128,
            }
        );

        if column.primary_key {
            view_column.set_title(
                Some(&format!("{} {}", &column.name, SYMBOL_PRIMARY_KEY))
            );
        }

        if i == columns.len() - 1 {
            view_column.set_expand(true);
        }

        column_view.append_column(&view_column);
    }

    column_view.set_show_column_separators(true);
    column_view.set_show_row_separators(true);

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&column_view));
    scrolled_window.set_vexpand(true);
    scrolled_window
}


fn setup_list_item(obj: &Object) -> Result<(), Box<dyn Error>> {
    let list_item = obj
        .downcast_ref::<ListItem>()
        .ok_or("Object is not a gtk4::ListItem")?;

    let label = Label::builder()
        .ellipsize(EllipsizeMode::End)
        .hexpand(true)
        .vexpand(true)
        .xalign(0.0)
        .margin_start(4)
        // .margin_top(2)
        .build();

    list_item.set_child(Some(&label));
    Ok(())
}
