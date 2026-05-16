//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gettextrs::gettext;

use gtk4::Window;

use libadwaita::prelude::*;
use libadwaita::{
    ShortcutsDialog,
    ShortcutsItem,
    ShortcutsSection,
};


pub fn show_shortcuts_dialog(parent: &Window){
    let shortcuts = ShortcutsDialog::new();

    shortcuts.add(section_general());
    shortcuts.add(section_table());

    shortcuts.present(Some(parent));
}


fn section_general() -> ShortcutsSection {
    let section = ShortcutsSection::new(Some(&gettext("General")));

    let item_open   = ShortcutsItem::new(&gettext("Open File"),     "<Primary>o");
    let item_menu   = ShortcutsItem::new(&gettext("Open Menu"),     "F10");
    let item_reload = ShortcutsItem::new(&gettext("Reload Window"), "<Primary>r");
    let item_close  = ShortcutsItem::new(&gettext("Close Window"),  "<Primary>w");
    let item_quit   = ShortcutsItem::new(&gettext("Quit"),          "<Primary>q");

    section.add(item_open);
    section.add(item_menu);
    section.add(item_reload);
    section.add(item_close);
    section.add(item_quit);

    section
}


fn section_table() -> ShortcutsSection {
    let section = ShortcutsSection::new(Some(&gettext("Tables")));

    let item_copy = ShortcutsItem::new(&gettext("Copy Row"), "<Primary>c");
    // let item_jump = ShortcutsItem::new("Jump To Row", "<Primary>l"); // TODO

    section.add(item_copy);
    // section.add(item_jump);

    section
}
