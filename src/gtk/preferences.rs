//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::gio::Settings;
use gtk4::{ StringList, Window };
use libadwaita::prelude::*;
use libadwaita::{
    ComboRow,
    PreferencesDialog,
    PreferencesGroup,
    PreferencesPage,
    SwitchRow,
};

// use super::sponsors::*;


pub fn show_preferences_dialog(parent: &Window, _page: Option<PreferencesPage>) {
    let settings = Settings::new("studio.planetpeanut.Bobby");

    let show_row_numbers: bool = settings.boolean("row-numbers");
    gtk4::glib::g_info!("settings", "Row numbers enabled: {}", show_row_numbers);

    let page = PreferencesPage::builder()
        .title("Preferences")
        .icon_name("org.gnome.Settings-system-symbolic")
        .build();


    let group_rows_columns = PreferencesGroup::builder()
        .title("Rows &amp; Columns")
        .build();



    let switch = SwitchRow::builder()
        .title("Row Numbers")
        .build();

    let _bind = settings.bind(
        "row-numbers",
        &switch,
        "active"
    );

    group_rows_columns.add(&switch);


    group_rows_columns.add(&row_order());
    group_rows_columns.add(&row_separator());


    let group_display = PreferencesGroup::builder()
        .title("Display")
        .build();

    group_display.add(&row_monospace());
    group_display.add(&row_binary());


    page.add(&group_rows_columns);
    page.add(&group_display);


    let preferences = PreferencesDialog::new();
    preferences.add(&page);
    // preferences.add(&sponsors_page());

    preferences.present(Some(parent));
}



// fn row_numbers(settings: &Settings) -> SwitchRow {
//     let switch = SwitchRow::builder()
//         .title("Row Numbers")
//         .build();

//     _ = settings.bind(
//         "row-numbers",
//         &switch,
//         "active"
//     );

//     switch
// }

fn row_order() -> ComboRow {
    ComboRow::builder()
        .title("Row Order")
        .model(&StringList::new(&[
            "Newest First",
            "Oldest First",
        ]))
        .build()
}

fn row_separator() -> ComboRow {
    ComboRow::builder()
        .title("Column Separator")
        .subtitle("Used when copying rows to the clipboard")
        .model(&StringList::new(&[
            "Tabs",
            "Spaces",
            "Commas",
            "Markdown",
        ]))
        .build()
}


fn row_monospace() -> SwitchRow {
    SwitchRow::builder()
        .title("Monospace Font")
        .build()
}

fn row_binary() -> ComboRow {
    ComboRow::builder()
        .title("Binary Preview")
        .subtitle("How to display columns with binary data")
        .model(&StringList::new(&[
            "Size in Bytes",
            "Hex Values",
        ]))
        .build()
}
