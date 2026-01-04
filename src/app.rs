//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU General Public License v3 or any later version.


use std::env;
use std::path::{ Path, PathBuf };
use std::process::Command;

use crate::log;


#[derive(Debug)]
pub struct App {
    pub id:      String,
    pub name:    String,
    pub command: String,
    pub icon:    String,
    pub version: String,

    // Docs: https://docs.flatpak.org/en/latest/sandbox-permissions.html
    pub is_flatpak: bool,

    // Installation
    pub prefix:       PathBuf,
    pub desktop_file: PathBuf,
    pub appdata_file: PathBuf,

    // Runtime
    pub app_config_home: PathBuf,
    pub app_data_home:   PathBuf,
    pub app_cache_home:  PathBuf,
}


impl Default for App {
    fn default() -> Self {
        let app_id = "studio.planetpeanut.Bobby";

        let app_name     = env!("CARGO_PKG_NAME");
        let command_name = env!("CARGO_BIN_NAME");
        let version      = env!("CARGO_PKG_VERSION");

        let prefix = Path::new("/app");


        // HOME
        let home_dir = env::var("HOME").unwrap_or_else(|_| {
            log::error_and_exit("Could not read HOME environment variable")
        });

        let home_dir = Path::new(&home_dir);


        // TODO: GSETTINGS SCHEMA
        // GSETTINGS_SCHEMA_DIR=./data if not set


        // XDG
        let mut xdg_config_home   = home_dir.join(".config");      // ~/.var/app/<APP_ID>/config
        let mut xdg_data_home     = home_dir.join(".local/share"); // ~/.var/app/<APP_ID>/data
        let mut xdg_cache_home    = home_dir.join(".cache");       // ~/.var/app/<APP_ID>/cache

        // Flatpak
        if let Ok(var) = env::var("XDG_CONFIG_HOME") { xdg_config_home = Path::new(&var).into(); }
        if let Ok(var) = env::var("XDG_DATA_HOME") { xdg_data_home = Path::new(&var).into(); }
        if let Ok(var) = env::var("XDG_CACHE_HOME") { xdg_cache_home = Path::new(&var).into(); }

        App {
            id:      app_id.into(),
            name:    app_name.into(),
            command: command_name.into(),
            icon:    app_id.into(),
            version: version.into(),

            is_flatpak: app_is_flatpak(),

            // Installation
            prefix:       prefix.into(),
            desktop_file: prefix.join(format!("share/applications/{command_name}.desktop")),
            appdata_file: prefix.join(format!("share/appdata/{command_name}.appdata.xml")),

            // Runtime
            app_config_home: xdg_config_home.join(command_name),
            app_data_home:   xdg_data_home.join(command_name),
            app_cache_home:  xdg_cache_home.join(command_name),
        }
    }
}


pub fn app_runs_as_root() -> bool {
    match Command::new("id").arg("-u").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "0",
        Err(_) => true, // Assume root if we can't check
    }
}

pub fn app_runs_in_terminal() -> bool {
    // Propagated even in Flatpak
    env::var("TERM").is_ok()
}


pub fn app_version() -> String {
    let s = format!("{} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"));

    if app_is_flatpak() {
        format!("{s} (Flatpak)")
    } else {
        s
    }
}


pub fn app_deps() -> String {
    format!("SQLite {}", rusqlite::version())
}


pub fn app_is_flatpak() -> bool {
    // Docs: https://docs.flatpak.org/en/latest/flatpak-command-reference.html#flatpak-run
    env::var("FLATPAK_ID").is_ok()
}
