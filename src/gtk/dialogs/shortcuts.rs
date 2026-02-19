//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::{
    ShortcutsGroup,
    ShortcutsSection,
    ShortcutsShortcut,
    ShortcutsWindow,
    Window,
};


pub fn show_shortcuts_dialog(parent: &Window){
    let section_general = ShortcutsSection::builder()
        .title("General")
        .build();

    let group_general = ShortcutsGroup::builder().build();

    let item_open = ShortcutsShortcut::builder()
        .title("Open File")
        .accelerator("<Primary>o")
        .build();

    let item_close = ShortcutsShortcut::builder()
        .title("Close Window")
        .accelerator("<Primary>w")
        .build();

    let item_quit = ShortcutsShortcut::builder()
        .title("Quit")
        .accelerator("<Primary>q")
        .build();

    group_general.append(&item_open);
    group_general.append(&item_close);
    group_general.append(&item_quit);
    section_general.append(&group_general);

    let shortcuts = ShortcutsWindow::builder()
        .transient_for(parent)
        .modal(true)
        .build();

    shortcuts.add_section(&section_general);
    shortcuts.present();
}
