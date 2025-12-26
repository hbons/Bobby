//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::{ License, Window, };
use libadwaita::AboutWindow;


pub fn show_about_dialog(parent: &Window) {
    let about = AboutWindow::builder() // TODO: from_appdata() if running in Flatpak?
        .transient_for(parent)
        .application_name(env!["CARGO_PKG_NAME"])
        .application_icon("studio.planetpeanut.Bobby")
        .developer_name("Hylke Bons")
        .version(env!["CARGO_PKG_VERSION"])
        // .developers(["Hylke Bons"])
        // .designers(["Hylke Bons"])
        .license_type(License::Gpl30)
        .website(env!["CARGO_PKG_HOMEPAGE"])
        .issue_url(env!["CARGO_PKG_REPOSITORY"])
        .build();

    about.present();
}
