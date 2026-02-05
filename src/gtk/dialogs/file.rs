//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::{
    glib::Error,
    Cancellable,
    File,
    ListModel,
    ListStore,
};

use gtk4::prelude::*;
use gtk4::{
    FileDialog,
    FileFilter,
    Window,
};

use crate::gtk::windows::window::try_window_new;
use crate::gtk::windows::window_empty::IS_EMPTY_WINDOW;


pub fn show_file_dialog(parent: &Window) {
    let dialog = FileDialog::builder()
        .filters(&filters())
        .modal(true)
        .build();

    let parent_handle = parent.clone();

    dialog.open_multiple(
        Some(parent),
        Some(&Cancellable::new()),
        move |result| {
            if let Err(e) = handle_files(&parent_handle, result) {
                // TODO: Show error on StatusPage
                eprintln!("Failed to open files: {e}");
            }
        },
    );
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


fn handle_files(
    parent: &Window,
    result: Result<ListModel, Error>)
    -> Result<(), Box<dyn std::error::Error>>
{
    let model = result?;

    let application = parent.application()
        .ok_or("Missing application in Window")?
        .downcast::<libadwaita::Application>()
        .map_err(|_| "Not a libadwaita::Application")?;

    if parent.widget_name() == IS_EMPTY_WINDOW {
        parent.close();
    }

    for i in 0..model.n_items() {
        let file = model
            .item(i)
            .and_then(|obj| obj.downcast::<File>().ok())
            .ok_or("ListModel item is not a gio::File")?;

        let path = file
            .path()
            .ok_or("Selected file has no local path")?;

        if let Some(window) = application
            .windows()
            .iter()
            .find(|w| w.widget_name().to_string() == path.to_string_lossy())
        {
            window.present();
        } else {
            try_window_new(&application, &file, false);
        }
    }

    Ok(())
}
