//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gtk4::prelude::*;
use gtk4::{
    gdk::Display,
    Widget,
};


// TODO: Use generics and return Option<T>
pub fn widget_by_name(name: &str, parent: &Widget) -> Option<Widget> {
    if parent.widget_name() == name {
        return Some(parent.clone());
    }

    let mut child = parent.first_child();

    while let Some(widget) = child {
        if let Some(found) = widget_by_name(name, &widget) {
            return Some(found);
        }

        child = widget.next_sibling();
    }

    None
}


pub fn copy_to_clipboard(s: &str) -> Result<(), Box<dyn Error>> {
    let display = Display::default()
        .ok_or("Missing Display")?;

    let clipboard = display.clipboard();
    clipboard.set_text(s);

    Ok(())
}


// TODO: Use widget_by_name() everywhere
pub fn find_column_view(root: &Widget) -> Option<gtk4::ColumnView> {
    if let Ok(column_view) = root.clone().downcast::<gtk4::ColumnView>() {
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
