//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::{ StringList, Window };
use gtk4::gio::{ Settings, SettingsSchemaSource };
use gtk4::glib::Variant;

use libadwaita::prelude::*;
use libadwaita::{
    ComboRow,
    PreferencesDialog,
    PreferencesGroup,
    PreferencesPage,
    SwitchRow,
};


pub fn show_preferences_dialog(parent: &Window) {
    let settings = Settings::new("studio.planetpeanut.Bobby");

    let page = PreferencesPage::builder()
        .title("Preferences")
        .icon_name("org.gnome.Settings-system-symbolic")
        .build();


    let group_rows_columns = PreferencesGroup::builder()
        .title("Rows &amp; Columns")
        .build();

    group_rows_columns.add(&row_numbers(&settings));
    group_rows_columns.add(&row_order(&settings));
    group_rows_columns.add(&row_separator(&settings));


    let group_appearance = PreferencesGroup::builder()
        .title("Appearance")
        .build();

    group_appearance.add(&row_monospace(&settings));

    page.add(&group_rows_columns);
    page.add(&group_appearance);


    let preferences = PreferencesDialog::new();
    preferences.add(&page);

    preferences.present(Some(parent));
}


fn row_numbers(settings: &Settings) -> SwitchRow {
    let switch = SwitchRow::builder()
        .title("Row Numbers")
        .build();

    settings.bind(
        "row-numbers",
        &switch,
        "active"
    ).build();

    switch
}

fn row_order(settings: &Settings) -> ComboRow {
    combo_row_with_binding(
        settings,
        "row-order",
        "Row Order",
        None,
        &["Newest First",
        "Oldest First"],
    )
}

fn row_separator(settings: &Settings) -> ComboRow {
    combo_row_with_binding(
        settings,
        "column-separator",
        "Column Separator",
        Some("Used when copying rows to the clipboard"),
        &["Tabs", "Spaces", "Commas", "Markdown"],
    )
}


fn row_monospace(settings: &Settings) -> SwitchRow {
    let switch = SwitchRow::builder()
        .title("Monospace Font")
        .build();

    settings.bind(
        "monospace-font",
        &switch,
        "active"
    ).build();

    switch
}


fn combo_row_with_binding(
    settings: &Settings,
    key: &str,
    title: &str,
    subtitle: Option<&str>,
    choices: &[&str],
) -> ComboRow
{
    let value = settings.string(key).to_string();
    let index = value_to_index(settings, key, &value);

    let combo = ComboRow::builder()
        .title(title)
        .subtitle(subtitle.unwrap_or(""))
        .model(&StringList::new(choices))
        .build();

    if let Some(i) = index {
        combo.set_selected(i);
    }

    let key_handle = key.to_string();
    let settings_handle = settings.clone();

    combo.connect_notify_local(Some("selected"), move |row, _| {
        let index = row.selected();

        if let Some(value) = index_to_value(&settings_handle, &key_handle, index) {
            _ = settings_handle.set_string(&key_handle, &value);
        }
    });

    combo
}


fn choices_for_key(settings: &Settings, key: &str) -> Option<Vec<String>> {
    let source = SettingsSchemaSource::default()?;
    let schema = source.lookup(settings.schema_id()?.as_ref(), true)?;
    let schema_key = schema.key(key);

    let range = schema_key.range();
    let (_kind, value): (String, Variant) = range.get()?;

    value.get()
}

fn index_to_value(settings: &Settings, key: &str, index: u32) -> Option<String> {
    let choices = choices_for_key(settings, key)?;
    choices.get(index as usize).cloned()
}

fn value_to_index(settings: &Settings, key: &str, value: &str) -> Option<u32> {
    let choices = choices_for_key(settings, key)?;
    let index = choices.iter().position(|v| v == value)?;

    Some(index as u32)
}
