//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gio::{
    Menu,
    Settings,
};

use gtk4::prelude::*;
use gtk4::{
    gdk::BUTTON_SECONDARY,
    gdk::Rectangle,
    ColumnView,
    ColumnViewColumn,
    GestureClick,
    Label,
    ListTabBehavior,
    PopoverMenu,
    PickFlags,
    ScrolledWindow,
    SignalListItemFactory,
    SingleSelection,
};

use crate::bobby::prelude::*;
use crate::bobby::sqlite::cache::DatabaseCacheModel;

use super::item::{
    bind_index_list_item,
    bind_list_item,
    setup_index_list_item,
    setup_list_item,
    SYMBOL_KEY,
};


pub fn content_new(
    database: &Database,
    table: &Table,
) -> Result<ScrolledWindow, Box<dyn Error>>
{
    let settings = Settings::new("studio.planetpeanut.Bobby"); // TODO
    let monospace_font: bool = settings.get("monospace-font");

    let model = DatabaseCacheModel::from_database(database, table);
    let selection = SingleSelection::new(Some(model));

    let column_view = ColumnView::builder()
        .enable_rubberband(false)
        .focusable(true)
        .has_tooltip(true)
        .model(&selection)
        .reorderable(false)
        .show_column_separators(true)
        .show_row_separators(true)
        .single_click_activate(true)
        .tab_behavior(ListTabBehavior::Cell)
        .build();

    let row_count = database.row_count(table)?;
    let columns = database.columns(table)?;
    let mut columns = columns.clone();
    columns.insert(0, Column::default()); // Reserve for row numbers

    for (column_index, column) in columns.iter().enumerate() {
        let is_index_column = column_index == 0;
        let is_last_column = column_index == columns.len() - 1;

        let factory = SignalListItemFactory::new();

        unsafe {
            factory.set_data("column", column_index.clone());
        }

        if is_index_column {
            factory.connect_setup(move |_factory, obj| {
                if let Err(e) = setup_index_list_item(obj) {
                    eprintln!("Failed to set up index list item: {e}");
                }
            });

            factory.connect_bind(move |_factory, obj| {
                if let Err(e) = bind_index_list_item(obj, row_count) {
                    eprintln!("Failed to bind index list item: {e}");
                }
            });
        } else {
            let primary_key = column.primary_key;

            factory.connect_setup(move |_factory, obj| {
                if let Err(e) = setup_list_item(obj, monospace_font) {
                    eprintln!("Failed to set up list item: {e}");
                }
            });

            factory.connect_bind(move |_factory, obj| {
                if let Err(e) = bind_list_item(obj, column_index, primary_key) {
                    eprintln!("Failed to bind index list item: {e}");
                }
            });
        }

        let view_column = ColumnViewColumn::builder()
            .title(&column.name)
            .id(&column_index.to_string())
            .factory(&factory)
            .resizable(true)
            .expand(is_last_column)
            .build();

        if column.primary_key {
            let title = format!("{} {}", &column.name, SYMBOL_KEY);
            view_column.set_title(Some(&title));
        }

        if is_index_column {
            const CHAR_WIDTH: usize = 12;
            let margin_end = super::item::MARGIN_END as usize;

            let width = ((row_count.to_string().len() + 1) * CHAR_WIDTH) + margin_end;
            view_column.set_fixed_width(width as i32);
            view_column.set_resizable(false);

            // TODO: File GTK rendering bug
            settings.bind(
                "row-numbers",
                &view_column,
                "visible"
            ).build();
        } else {
            view_column.set_fixed_width(
                match column.affinity {
                    Affinity::BLOB(_, _) => { view_column.set_resizable(false); 128 },
                    Affinity::TEXT(_) => 192,
                    _ => 128,
                }
            );
        }

        column_view.append_column(&view_column);
    }


    let click = GestureClick::builder()
        .button(BUTTON_SECONDARY)
        .build();

    let column_view_handle = column_view.clone();

    click.connect_pressed(move |gesture, _n_presses, x, y| {
        if let Err(e) = content_clicked(gesture, x, y, &column_view_handle) {
            eprintln!("Failed to open context menu: {e}");
        }
    });


    column_view.add_controller(click);
    column_view.grab_focus();

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&column_view));
    scrolled_window.set_vexpand(true);

    Ok(scrolled_window)
}


fn content_clicked(
    gesture: &GestureClick,
    x: f64,
    y: f64,
    column_view: &ColumnView,
) -> Result<(), Box<dyn Error>>
{
    let model = column_view
        .model()
        .ok_or("Missing model on ColumnView")?;

    let single_selection = model
        .downcast::<SingleSelection>()
        .map_err(|_| "Model is not a gtk4::SingleSelection")?;

    let picked = column_view
        .pick(x, y, PickFlags::NON_TARGETABLE)
        .ok_or("Could not pick Widget")?;

    // TODO: Also handle label parent if internal margin clicked
    let label = picked
        .downcast::<Label>()
        .map_err(|_| "Widget is not a gtk4::Label")?;

    let row = single_selection.selected() as usize;
    let col = label.widget_name().parse::<usize>()?;

    if let Some(col) = col.checked_sub(1) {
        context_menu_open(gesture, col, row, x, y);
    }

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

        // TODO: GTK warning when switching tables after opening context menu
        popover.set_parent(&widget);
        popover.popup();
    }
}
