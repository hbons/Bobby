//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::Window;

use libadwaita::prelude::*;
use libadwaita::{
    ShortcutsDialog,
    ShortcutsItem,
    ShortcutsSection,
};


pub fn show_shortcuts_dialog(parent: &Window){
    // let table_section = ShortcutsSection::new(Some("Tables"));

    // let item_goto = ShortcutsItem::new("Go To Row Number", "<Ctrl>G");
    // let item_copy = ShortcutsItem::new("Copy Row", "<Ctrl>C");

    // table_section.add(item_goto); // TODO
    // table_section.add(item_copy); // TODO


    let general_section = ShortcutsSection::new(Some("General"));

    let item_open = ShortcutsItem::new("Open File", "<Ctrl>O");
    let item_close = ShortcutsItem::new("Close Window", "<Ctrl>W");
    let item_menu = ShortcutsItem::new("Open Menu", "F10");
    let item_quit = ShortcutsItem::new("Quit", "<Ctrl>Q");

    general_section.add(item_open);
    general_section.add(item_close);
    general_section.add(item_menu);
    general_section.add(item_quit);


    let shortcuts = ShortcutsDialog::new();
    // shortcuts.add(table_section);
    shortcuts.add(general_section);

    shortcuts.present(Some(parent));
}
