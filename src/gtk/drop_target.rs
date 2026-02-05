//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::{
    gdk::DragAction,
    gdk::FileList,
    DropTarget,
};

use libadwaita::ApplicationWindow;

use crate::gtk::windows::window_empty::IS_EMPTY_WINDOW;


pub fn drop_target_new(window: &ApplicationWindow) -> DropTarget {
    let drop_target = DropTarget::new(
        FileList::static_type(),
        DragAction::COPY,
    );

    let window_handle = window.clone();

    drop_target.connect_drop(move |_, value, _, _| {
        if window_handle.widget_name() == IS_EMPTY_WINDOW {
            window_handle.set_visible(false); // Keeps reference alive
        }

        if let Some(application) = window_handle.application() {
            if let Ok(list) = value.get::<FileList>() {
                application.open(&list.files(), "");
            }
        }

        if window_handle.widget_name() == IS_EMPTY_WINDOW {
            window_handle.close(); // Now we can close it
        }

        true // Drop accepted
    });

    drop_target
}
