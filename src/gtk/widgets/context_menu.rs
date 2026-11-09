//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gio::Menu;

use gtk4::prelude::*;
use gtk4::{
    gdk::Rectangle,
    glib,
    GestureClick,
    PopoverMenu,
};


pub fn open(
    gesture: &GestureClick,
    col_index: usize,
    row_index: usize,
    x: f64,
    y: f64,
) {
    let Some(widget) = gesture.widget() else {
        return;
    };

    let edit_section = Menu::new();
    let copy_section = Menu::new();
    let delete_section = Menu::new();

    edit_section.append(
        Some("_Edit…"),
        Some(&format!("win.edit-val::{}:{}", row_index, col_index)),
    );

    copy_section.append(
        Some("_Copy"),
        Some(&format!("win.copy-val::{}:{}", row_index, col_index)),
    );

    copy_section.append(
        Some("Copy _Row"),
        Some("win.copy-row"),
    );

    delete_section.append(
        Some("_Delete Row"),
        Some("win.delete-row"),
    );

    let menu = Menu::new();
    menu.append_section(None, &edit_section);
    menu.append_section(None, &copy_section);
    menu.append_section(None, &delete_section);

    let popover = PopoverMenu::builder()
        .has_arrow(false)
        .menu_model(&menu)
        .pointing_to(&Rectangle::new(x as i32, y as i32, 0, 0))
        .build();

    popover.set_parent(&widget);

    popover.connect_closed(move |w| {
        let widget = w.clone();

        // Prevent menu closing before the action is triggered
        glib::idle_add_local_once(move || {
            widget.unparent();
        });
    });

    // Fix to prevent scrollbars
    glib::idle_add_local_once(move || {
        popover.popup();
    });
}
