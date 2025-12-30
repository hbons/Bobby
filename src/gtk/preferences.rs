//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


// use gtk4::gio::Settings;
use gtk4::{ StringList, Window };
use libadwaita::ComboRow;
use libadwaita::prelude::AdwDialogExt;
use libadwaita::prelude::PreferencesDialogExt;
use libadwaita::prelude::PreferencesGroupExt;
use libadwaita::prelude::PreferencesPageExt;
use libadwaita::{
    PreferencesDialog,
    PreferencesGroup,
    PreferencesPage,
    SwitchRow,
};


pub fn show_preferences_dialog(parent: &Window) {
    let preferences = PreferencesDialog::builder()
        .build();


    // let _settings = Settings::new("studio.planetpeanut.Bobby");


    let row1 = SwitchRow::builder()
        .title("Row Numbers")
        .build();


    let row3 = SwitchRow::builder()
        .title("Monospace Font")
        .build();



    let group = PreferencesGroup::builder()
        // .title("Display Options")
        .build();

    let model = StringList::new(&[
        "Newest First",
        "Oldest First",
    ]);

    let row2 = ComboRow::builder()
        .title("Row Order")
        .model(&model)
        .build();



    let model = StringList::new(&[
        "Tabs",
        "Spaces",
        "Commas",
        "Markdown",
    ]);

    let row4 = ComboRow::builder()
        .model(&model)
        .title("Column Separator")
        .subtitle("Used when copying rows to the clipboard")
        .build();



    let model = StringList::new(&[
        "Size in Bytes",
        "Hex Values",
    ]);


    let row5 = ComboRow::builder()
        .model(&model)
        .title("Binary Preview")
        .subtitle("How columns with binary data are displayed")
        .build();

    let page = PreferencesPage::builder()
        .title("General")
        .icon_name("emblem-system-symbolic")
        .build();

    // let page2 = PreferencesPage::builder()
    //     .title("Sponsor")
    //     .icon_name("emote-love-symbolic")
    //     .build();


    group.add(&row1);
    group.add(&row2);
    group.add(&row4);
    group.add(&row3);
    group.add(&row5);

    page.add(&group);

    preferences.add(&page);
    // preferences.add(&page2);

    preferences.present(Some(parent));
}
