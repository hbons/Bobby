//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;
use std::path::Path;

use gio::SimpleAction;
use gtk4::{ScrolledWindow, prelude::*};
use gtk4::glib::Variant;
use gtk4::{
    Align,
    Button,
    CenterBox,
    Orientation,
};

use libadwaita::prelude::*;
use libadwaita::{
    Application,
    ApplicationWindow,
    ButtonContent,
    HeaderBar,
};

use crate::bobby::prelude::*;
use super::content::content_new;
use super::file::open_file_dialog;
use super::menu::main_menu_new;
use super::switcher::table_switcher_new;


pub fn window_empty_new(application: &Application) -> Result<ApplicationWindow, Box<dyn Error>> {
    let window = ApplicationWindow::builder()
        .application(application)
        .default_height(360)
        .default_width(540)
        .resizable(false)
        .title("Bobby")
        .build();

    let header = HeaderBar::new();
    header.pack_end(&main_menu_new(application));

    let button = button_open_new(&window);
    let center = CenterBox::new();
    center.set_center_widget(Some(&button));
    center.set_vexpand(true);

    let layout = gtk4::Box::new(Orientation::Vertical, 0);
    layout.append(&header);
    layout.append(&center);

    window.set_content(Some(&layout));

    Ok(window)
}

fn button_open_new(window: &ApplicationWindow) -> Button {
    let content = ButtonContent::new();
    content.set_icon_name("folder-open-symbolic");
    content.set_label("Open Database");

    let button = Button::new();
    button.add_css_class("pill");
    button.add_css_class("suggested-action");
    button.set_child(Some(&content));
    button.set_valign(Align::Center);
    button.set_halign(Align::Center);

    let window_weak = window.downgrade();

    button.connect_clicked(move |_| {
        if let Some(window) = window_weak.upgrade() {
            open_file_dialog(&window);
        }
    });

    button
}


pub fn window_new(application: &Application, path: &Path, table_name: Option<String>) -> Result<ApplicationWindow, Box<dyn Error>> {
    let db = Database::from_file(path)?;
    let tables = db.tables()?;

    let table = match table_name {
        Some(name) => name.parse::<Table>()?,
        None => tables.first().cloned().ok_or("Missing table")?,
    };

    let title = &path.file_name()
        .ok_or("err")?
        .to_string_lossy()
        .to_string();

    let window = ApplicationWindow::builder()
        .application(application)
        .title(title)
        .default_width(720)
        .default_height(480)
        .build();

    let switcher = table_switcher_new(&tables);
    switcher.set_label(table.name());

    let main_menu = main_menu_new(application);

    let header = HeaderBar::new();
    header.pack_start(&switcher);
    header.pack_end(&main_menu);

    let content = content_new(
        &db.columns(&table)?,
        &db.rows(&table)?
    );

    let action = SimpleAction::new_stateful(
        "table",
        Some(&String::static_variant_type()),
        &Variant::from(table.name()),
    );

    let layout = gtk4::Box::new(Orientation::Vertical, 0);
    let layout_handle = layout.clone();
    let window_handle = window.clone();
    let switcher_handle = switcher.clone();

    action.connect_change_state(move |action, value| {
        if let Some(value_str) = value.and_then(|v| v.str()) {
            action.set_state(&Variant::from(value_str));
            switcher_handle.set_label(value_str);

            if let Ok(table) = value_str.parse::<Table>() {
                match window_change_content(&window_handle, &table) {
                    Ok(new_content) => {
                        if let Some(old_content) = layout_handle.last_child() {
                            layout_handle.remove(&old_content);
                            layout_handle.append(&new_content);
                        }
                    },
                    Err(e) => eprintln!("Could not change content: {e}"),
                };
            }
        }
    });

    layout.append(&header);
    layout.append(&content);

    window.set_content(Some(&layout));
    window.add_action(&action);

    // SAFETY: Window outlives the database
    unsafe {
        window.set_data("db", db);
    }

    Ok(window)
}


fn window_change_content(window: &ApplicationWindow, table: &Table)
    -> Result<ScrolledWindow, Box<dyn Error>> {

    // SAFETY: Window outlives the database
    let db = unsafe {
        window
            .data::<Database>("db")
            .map(|db| db.as_ref())
    };

    let db = db.ok_or("Database not found on window")?;

    let content = content_new(
        &db.columns(table)?,
        &db.rows(table)?
    );

    // TODO: Swap the content here. Need to get the layout box somehow...

    Ok(content)
}
