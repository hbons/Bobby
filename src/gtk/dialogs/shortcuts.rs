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
    // let section_table = ShortcutsSection::new(Some("Tables"));

    // let item_jump = ShortcutsItem::new("Jump To Row", "<Primary>l");
    // let item_copy = ShortcutsItem::new("Copy Row", "<Primary>c");
    // let item_refresh = ShortcutsItem::new("Refresh", "<Primary>r");

    // section_table.add(item_jump); // TODO
    // section_table.add(item_copy); // TODO
    // section_table.add(item_refresh); // TODO


    let section_general = ShortcutsSection::new(Some("General"));

    let item_open  = ShortcutsItem::new("Open File", "<Primary>o");
    // let item_menu  = ShortcutsItem::new("Open Menu", "F10"); // TODO
    let item_close = ShortcutsItem::new("Close Window", "<Primary>w");
    let item_quit  = ShortcutsItem::new("Quit", "<Primary>q");

    section_general.add(item_open);
    // section_general.add(item_menu);
    section_general.add(item_close);
    section_general.add(item_quit);


    let shortcuts = ShortcutsDialog::new();
    // shortcuts.add(table_section);
    shortcuts.add(section_general);
    shortcuts.present(Some(parent));
}
