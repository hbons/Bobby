//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::{ License, Window, };
use libadwaita::AboutDialog;
use libadwaita::prelude::AdwDialogExt;


pub fn show_about_dialog(parent: &Window) {
    let about = AboutDialog::builder() // TODO: from_appdata() if running in Flatpak?
        .application_name(env!["CARGO_PKG_NAME"])
        .application_icon("studio.planetpeanut.Bobby")
        .developer_name("Hylke Bons")
        .version(env!["CARGO_PKG_VERSION"])
        .debug_info(
            format!("SQLite {}", rusqlite::version()))
        .license_type(License::Gpl30)
        .website(env!["CARGO_PKG_HOMEPAGE"])
        .issue_url(env!["CARGO_PKG_REPOSITORY"])
        .build();

    about.present(Some(parent));
}
