//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::cell::Ref;
use std::error::Error;

use gio::File;

use gtk4::prelude::*;
use gtk4::{
    gdk::Display,
    glib::BoxedAnyObject,
    glib::Propagation,
    ColumnView,
    Orientation,
    ScrolledWindow,
    SingleSelection,
    Widget,
};

use libadwaita::prelude::*;
use libadwaita::{
    Application,
    ApplicationWindow,
    HeaderBar,
    ToastOverlay,
};

use crate::bobby::prelude::*;

use crate::gtk::actions::prelude::*;
use crate::gtk::widgets::content::content_new;
use crate::gtk::widgets::drop_target::drop_target_new;
use crate::gtk::widgets::menu::main_menu_new;
use crate::gtk::widgets::switcher::table_switcher_new;
use crate::gtk::windows::prelude::*;


pub fn try_window_new(application: &Application, file: &File, quit_on_close: bool) {
    let settings = gio::Settings::new("studio.planetpeanut.Bobby"); // TODO

    let row_order = match settings.string("row-order").as_str() {
        "newest-first" => Some(RowOrder::Descending),
        "oldest-first" => Some(RowOrder::Ascending),
        _ => None,
    };

    match Database::from_file(file, row_order) {
        Ok(db) => {
            if let Ok(window) = window_new(application, &db, None, quit_on_close) {
                window.present();
            }
        },
        Err(e) => {
            if let Some(path) = &file.path() &&
               let Ok(window) = window_error_new(application, path, e) {
                window.present();
            }
        }
    }
}


// TODO: Remove views and Database from memory on close
pub fn window_new(
    application: &Application,
    db: &Database,
    table_name: Option<String>,
    quit_on_close: bool
) -> Result<ApplicationWindow, Box<dyn Error>>
{
    let tables = db.tables()?;

    let table =
        if let Some(name) = table_name {
            tables
                .iter()
                .find(|t| t.name() == name)
                .cloned()
                .ok_or("Table does not exist")?
        } else {
            tables
                .first()
                .cloned()
                .ok_or("Table list empty")?
        };

    let path = db.file
        .path()
        .ok_or("Missing file path")?;

    let title = path.file_name()
        .ok_or("Missing file name")?
        .to_string_lossy()
        .to_string();

    let window = ApplicationWindow::builder()
        .application(application)
        .title(title)
        .default_width(960)
        .default_height(640)
        .build();

    // window.add_css_class("devel"); // TODO

    let switcher = table_switcher_new(&tables);
    switcher.set_label(&table.name());

    let main_menu = main_menu_new();

    let header = HeaderBar::new();
    header.pack_start(&switcher);
    header.pack_end(&main_menu);

    let content = content_new(db, &table)?;

    let table_index = tables
        .iter()
        .position(|t| t.name() == table.name())
        .ok_or("Table does not exist")?
        .to_string();

    let layout = gtk4::Box::new(Orientation::Vertical, 0);
    layout.append(&header);
    layout.append(&content);

    let overlay = ToastOverlay::new();
    overlay.set_child(Some(&layout));

    window.set_content(Some(&overlay));
    window.add_controller(drop_target_new(&window));
    window.set_widget_name(&path.to_string_lossy());

    window.add_action(&copy_row_action(&window, &overlay));
    window.add_action(&copy_val_action(&window, &overlay));
    window.add_action(&switch_table_action(&window, layout, table_index, tables, switcher));


    window.connect_close_request({
        let app = application.clone();

        move |_| {
            if !quit_on_close && app.windows().len() == 1 {
                // app.activate(); // TODO: Use activate logic
                if let Ok(empty_window) = window_empty_new(&app) {
                    empty_window.present();
                }
            }

            Propagation::Proceed
        }
    });


    // SAFETY: Window outlives the database
    unsafe {
        window.set_data("db", db.clone());
    }

    Ok(window)
}


pub fn copy_to_clipboard(s: &str) -> Result<(), Box<dyn Error>> {
    let display = Display::default().ok_or("Missing Display")?;

    let clipboard = display.clipboard();
    clipboard.set_text(s);

    Ok(())
}


pub fn window_change_content(window: &ApplicationWindow, table: &Table)
    -> Result<ScrolledWindow, Box<dyn Error>> {

    // SAFETY: Window outlives the database
    let db = unsafe {
        window
            .data::<Database>("db")
            .map(|db| db.as_ref())
    };

    let db = db.ok_or("Database not found on window")?;
    let content = content_new(db, table)?;

    // TODO: Swap the content here. Need to get the layout box somehow...

    Ok(content)
}


pub fn find_column_view(root: &Widget) -> Option<ColumnView> {
    if let Ok(column_view) = root.clone().downcast::<ColumnView>() {
        return Some(column_view);
    }

    let mut child = root.first_child();

    while let Some(widget) = child {
        if let Some(found) = find_column_view(&widget) {
            return Some(found);
        }

        child = widget.next_sibling();
    }

    None
}


pub fn get_row(column_view: ColumnView, position: usize) -> Option<Row> {
    let model = column_view.model()?;
    let selection = model.downcast_ref::<SingleSelection>()?;

    let item = selection
        .item(position as u32)
        .and_then(|o| o.downcast::<BoxedAnyObject>().ok())?;

    let row: Ref<Row> = item.borrow();
    let row = row.clone();

    Some(row)
}
