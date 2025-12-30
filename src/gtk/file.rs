//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::{ FileDialog, FileFilter };
use gtk4::gio::{ Cancellable, File, ListStore };
use gtk4::glib::Error;
use libadwaita::ApplicationWindow;

use super::window::window_new;


pub fn open_file_dialog(parent: &ApplicationWindow) {
    let dialog = FileDialog::builder()
        .filters(&filters())
        .modal(true)
        .build();

    let parent_handle = parent.clone();

    dialog.open(
        Some(parent),
        Some(&Cancellable::new()),
        move |result| {
            if let Err(e) = handle_file(&parent_handle, result) {
                eprintln!("Failed to open file: {e}");
            }
        },
    );
}

fn handle_file(
    parent: &ApplicationWindow,
    result: Result<File, Error>)
    -> Result<(), Box<dyn std::error::Error>> {

    let path = result
        .ok()
        .and_then(|file| file.path())
        .ok_or("No file selected")?;

    let application = parent.application()
        .ok_or("Missing application in Window")?;

    let application = application
        .downcast_ref::<libadwaita::Application>()
        .ok_or("Not a libadwaita::Application")?;

    parent.close();
    let window = window_new(application, path.as_path(), None)?;
    window.present();

    Ok(())
}


fn filters() -> ListStore {
    let filters = ListStore::new::<FileFilter>();
    filters.append(&filter_sqlite_files());
    filters.append(&filter_all_files());
    filters
}


fn filter_sqlite_files() -> FileFilter {
    let filter = FileFilter::new();
    filter.set_name(Some("SQLite Databases"));

    filter.add_mime_type("application/x-sqlite3");
    filter.add_mime_type("application/vnd-sqlite3");

    filter.add_pattern("*.db");
    filter.add_pattern("*.db3");
    filter.add_pattern("*.sqlite");
    filter.add_pattern("*.sqlite3");
    filter
}


fn filter_all_files() -> FileFilter {
    let filter = FileFilter::new();
    filter.set_name(Some("All Files"));
    filter.add_pattern("*");
    filter
}
