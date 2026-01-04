//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gio::{
    ListStore,
    Menu,
    Settings
};

use gtk4::prelude::*;
use gtk4::{
    gdk::BUTTON_SECONDARY,
    gdk::Rectangle,
    glib::Object,
    pango::EllipsizeMode,
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

use crate::bobby::prelude::*;


// U+25C7 "White Diamond"
const SYMBOL_PRIMARY_KEY: &str = "◇";


pub fn content_new(columns: &Vec<Column>, rows: &Vec<Row>) -> ScrolledWindow {
    let store = ListStore::new::<Row>();
    let row_count = rows.len();

    for row in rows.iter().take(100_000) {
        store.append(row); // TODO: Remove hard limit when we have lazy loading
    }

    let selection = SingleSelection::new(Some(store));
    let column_view = ColumnView::new(Some(selection));

    let mut columns = columns.clone();
    columns.insert(0, Column::default()); // Reserve for row numbers

    for (i, column) in columns.iter().enumerate() {
        let is_index_column = i == 0;
        let is_last_column = i == columns.len() - 1;

        let factory = SignalListItemFactory::new();
        let affinity = column.affinity;


        factory.connect_setup(move |_, obj| {
            if let Err(e) = setup_list_item(obj) {
                eprintln!("Failed to set up list item: {e}");
            }
        });


        let column_handle = column.clone();

        // TODO: Move out
        factory.connect_bind(move |_, obj| {
            if let Some(list_item) = obj.downcast_ref::<ListItem>() &&
               let Some(row) = list_item.item().and_downcast::<Row>()
            {
                let mut cells = row.cells();

                let row_number = (list_item.position() as usize) + 1;
                cells.insert(0, row_number.to_string());

                let text = cells
                    .get(i)
                    .map(String::as_str)
                    .unwrap_or_default();

                if let Some(label) = list_item.child().and_downcast::<Label>() {
                    // Reset state first to avoid rendering issues
                    label.set_sensitive(true);

                    // New state
                    if affinity == Affinity::INTEGER {
                        label.set_text(&thousands_sep(text));
                    } else {
                        label.set_text(text);
                    }

                    let mut tooltip = String::new();

                    if affinity == Affinity::BLOB {
                        let (length, hex_values) = text.split_once(": ").unwrap(); // TODO
                        label.set_sensitive(false);

                        let settings = gio::Settings::new("studio.planetpeanut.Bobby"); // TODO

                        // TODO: Bind to change
                        match settings.string("binary-preview").as_str() {
                            "size-bytes" => {
                                label.set_text(&format!("{length} BYTES"));
                                tooltip = format!("{affinity:?}  {length} BYTES");
                            },
                            "hex-values" => {
                                label.set_text(hex_values);
                                tooltip = format!("{affinity:?}  16 / {length} BYTES");
                            },
                            _ => {},
                        };
                    } else if column_handle.primary_key {
                        tooltip = format!("{SYMBOL_PRIMARY_KEY} PRIMARY KEY  {affinity:?}  {text}");
                    } else {
                        tooltip = format!("{affinity:?}  {text}");
                    }


                    let gesture = GestureClick::builder()
                        .button(BUTTON_SECONDARY)
                        .build();

                    let list_item_handle = list_item.clone();

                    // TODO: Check performance impact of this
                    gesture.connect_pressed(move |gesture, _, x, y| {
                        if let Some(widget) = gesture.widget() {
                            if let Some(_picked) = widget.pick(x, y, PickFlags::DEFAULT) {
                                let position = list_item_handle.position();

                                let col = i;
                                let row = position as usize;
                                context_menu_open(gesture, col, row, x, y);
                            }
                        }
                    });


                    if text == "NULL" {
                        label.set_sensitive(false);
                    }

                    if is_index_column {
                        tooltip = format!("{a} / {row_count}").into();
                        label.add_css_class("monospace");
                        label.set_ellipsize(EllipsizeMode::Start);
                        label.set_halign(Align::End);
                        label.set_margin_end(2);
                        label.set_margin_top(1);
                        label.set_sensitive(false);
                    }

                    let settings = Settings::new("studio.planetpeanut.Bobby"); // TODO

                    // TODO: Bind to change
                    if settings.boolean("monospace-font") {
                        label.add_css_class("monospace");
                        label.set_margin_top(1);
                    }

                    if let Some(cell) = label.parent() {
                        cell.add_controller(gesture);
                        cell.set_tooltip_text(Some(&tooltip));
                    }
                }
            }
        });


        // TODO: Sorting
        let view_column = ColumnViewColumn::builder()
            .title(&column.name)
            .factory(&factory)
            .resizable(true)
            .expand(is_last_column)
            .build();

        if column.primary_key {
            let title = format!("{} {}", &column.name, SYMBOL_PRIMARY_KEY);
            view_column.set_title(Some(&title));
        }

        if is_index_column {
            // Holds numbers up to 100k without ellipsis
            view_column.set_fixed_width(64);

            // TODO: File GTK rendering bug
            Settings::new("studio.planetpeanut.Bobby")
                .bind(
                    "row-numbers",
                    &view_column,
                    "visible"
                ).build();
        } else {
            view_column.set_fixed_width(
                match affinity {
                    Affinity::BLOB => { view_column.set_resizable(false); 128 }, // TODO: Check width
                    Affinity::TEXT => 192,
                    _ => 128,
                }
            );
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
        .halign(Align::Start)
        .margin_start(4)
        .build();

    list_item.set_child(Some(&label));

    Ok(())
}


fn context_menu_open(gesture: &GestureClick, col_index: usize, row_index: usize, x: f64, y: f64) {
    if let Some(widget) = gesture.widget() {
        let menu = Menu::new();

        menu.append(
            Some("Copy"),
            Some(&format!("win.copy-val::{}:{}", row_index, col_index)),
        );

        menu.append(
            Some("Copy Row"),
            Some(&format!("win.copy-row::{}", row_index))
        );

        // TODO
        // menu.append(
        //     Some("Copy Rows"),
        //     Some(&format!("win.copy-rows::{}", row_index))
        // );

        let popover = PopoverMenu::builder()
            .has_arrow(false)
            .menu_model(&menu)
            .pointing_to(&Rectangle::new(x as i32, y as i32, 0, 0))
            .build();

        popover.set_parent(&widget);
        popover.popup();
    }
}


fn thousands_sep(s: &str) -> String {
    let original = s;

    let (sign, digits) = s.strip_prefix('-')
        .map(|d| ("-", d))
        .unwrap_or(("", s));

    if digits.len() < 5 {
        return original.into();
    }

    let mut out = String::new();
    let mut count = 0;

    for ch in digits.chars().rev() {
        if count == 3 {
            out.push(',');
            count = 0;
        }
        out.push(ch);
        count += 1;
    }

    format!("{}{}", sign, out.chars().rev().collect::<String>())
}
