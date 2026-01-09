//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::cell::Ref;
use std::error::Error;

use gio::{
    Menu,
    Settings
};

use gtk4::prelude::*;
use gtk4::{
    gdk::BUTTON_SECONDARY,
    gdk::Rectangle,
    glib::BoxedAnyObject,
    glib::Object,
    pango::EllipsizeMode,
    Align,
    ColumnView,
    ColumnViewColumn,
    GestureClick,
    Label,
    ListItem,
    PopoverMenu,
    ScrolledWindow,
    SignalListItemFactory,
    SingleSelection,
};

use crate::bobby::prelude::*;
use crate::bobby::sqlite::cache::DatabaseCacheModel;


// U+25C7 "White Diamond"
const SYMBOL_PRIMARY_KEY: &str = "◇";


pub fn content_new(database: &Database, table: &Table) -> ScrolledWindow {
    let row_count = database.row_count(table).unwrap(); // TODO
    let columns = database.columns(table).unwrap(); // TODO

    let model = DatabaseCacheModel::from_database(&database, &table);
    let selection = SingleSelection::new(Some(model));
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

        let settings = Settings::new("studio.planetpeanut.Bobby");

        let column_handle = column.clone();
        let settings_handle = settings.clone();

        // TODO: Move out
        factory.connect_bind(move |_, obj| {
            if let Some(list_item) = obj.downcast_ref::<ListItem>() &&
               let Some(boxed) = list_item.item().and_downcast::<BoxedAnyObject>()
            {
                let row: Ref<Row> = boxed.borrow();
                let mut cells = row.cells.clone(); // TODO: try Rc so no clone needed

                let row_number = (list_item.position() as usize) + 1;
                cells.insert(0, row_number.to_string());

                let text = cells
                    .get(i)
                    .map(String::as_str)
                    .unwrap_or_default();

                if let Some(label) = list_item.child().and_downcast::<Label>() {
                    // Reset state first to avoid rendering issues
                    // label.set_sensitive(true);

                    // New state

                    if affinity == Affinity::INTEGER {
                        // label.set_text(&format_thousands(text));
                        label.set_text(text);

                    } else {
                        label.set_text(text);
                    }

                    // let mut tooltip = String::new();

                    if affinity == Affinity::BLOB {
                        label.set_sensitive(false);

                        if let Some((length, hex_values)) = text.split_once(":") {
                            label.set_text(length);
                            // tooltip = format!("{affinity:?}  {hex_values} …");
                        }
                    } else if column_handle.primary_key {
                        // tooltip = format!("{SYMBOL_PRIMARY_KEY} PRIMARY KEY  {affinity:?}  {text}");
                    } else {
                        // tooltip = format!("{affinity:?}  {text}");
                    }


                    // let gesture = GestureClick::new();
                    // gesture.set_button(BUTTON_SECONDARY);

                    // let list_item_handle = list_item.clone();

                    // // TODO: Check performance impact of this
                    // gesture.connect_pressed(move |gesture, _, x, y| {
                    //     let position = list_item_handle.position();
                    //     let row = position as usize;

                    //     // Note (hidden) row number column
                    //     if let Some(col) = i.checked_sub(1) {
                    //         context_menu_open(gesture, col, row, x, y);
                    //     }
                    // });


                    if text == "NULL" {
                        label.set_sensitive(false);
                    }

                    if is_index_column {
                        // tooltip = format!("{row_number} / {row_count}");
                        label.add_css_class("monospace");
                        label.set_ellipsize(EllipsizeMode::Start);
                        label.set_halign(Align::End);
                        label.set_margin_end(2);
                        label.set_margin_top(1);
                        label.set_sensitive(false);
                    }

                    // TODO: Bind to change
                    // if settings_handle.boolean("monospace-font") {
                    //     label.add_css_class("monospace");
                    //     label.set_margin_top(1);
                    // } // TODO: do in setup

                    // if let Some(cell) = label.parent() {
                        // cell.add_controller(gesture);
                        // cell.set_tooltip_text(Some(&tooltip));
                    // }
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
                    Affinity::BLOB => { view_column.set_resizable(false); 128 },
                    Affinity::TEXT => 192,
                    _ => 128,
                }
            );
        }

        if affinity == Affinity::NUMERIC {
            // TODO: Check if a date or datetime and fit column width
        }


        column_view.append_column(&view_column);
    }

    column_view.set_show_column_separators(true);
    column_view.set_show_row_separators(true);
    column_view.set_reorderable(false);

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
        .css_classes(["numeric"])
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

        // TODO: Also prepend column headers in Markdown mode
        // menu.append(
        //     Some("Copy Rows"),
        //     Some(&format!("win.copy-rows::{}", row_index))
        // );

        let popover = PopoverMenu::builder()
            .has_arrow(false)
            .menu_model(&menu)
            .pointing_to(&Rectangle::new(x as i32, y as i32, 0, 0))
            .build();

        // TODO: GTK warning when switching tables after opening menu
        popover.set_parent(&widget);
        popover.popup();
    }
}


fn format_thousands(s: &str) -> String {
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
