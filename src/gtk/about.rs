//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::{
    License,
    Window,
};

use libadwaita::prelude::*;
use libadwaita::AboutDialog;


pub fn show_about_dialog(parent: &Window) {
    let about = AboutDialog::builder() // TODO: from_appdata() if running in Flatpak?
        .application_icon("studio.planetpeanut.Bobby")
        .application_name(env!["CARGO_PKG_NAME"])
        .developer_name("Hylke Bons")
        .version(env!["CARGO_PKG_VERSION"])
        .website(env!["CARGO_PKG_HOMEPAGE"])
        .issue_url(env!["CARGO_PKG_REPOSITORY"])
        .copyright("Copyright © 2025–2026  Hylke Bons")
        .license_type(License::Gpl30)
        .debug_info(format!("SQLite {}", rusqlite::version()))
        .build();

    about.present(Some(parent));
}
