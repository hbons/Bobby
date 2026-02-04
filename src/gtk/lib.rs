//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gio::{
    ApplicationFlags,
    SimpleAction,
};

use gtk4::prelude::*;
use libadwaita::Application;

use crate::app::App;
use crate::gui::Gui;

use super::about::show_about_dialog;
use super::files::show_file_dialog;
use super::preferences::show_preferences_dialog;
use super::shortcuts::show_shortcuts_dialog;

use super::window::{
    window_empty_new,
    try_window_new,
};


impl Gui for App {
    // Docs: https://docs.gtk.org/gtk4
    //       https://gnome.pages.gitlab.gnome.org/libadwaita/doc

    fn gui_run(&self) -> Result<(), Box<dyn Error>> {
        let application = Application::builder()
            .application_id(&self.id)
            .flags(ApplicationFlags::HANDLES_OPEN)
            .build();

        application.connect_startup(|application| {
            // TODO: Move all tow actions.rs

            let preferences_action = SimpleAction::new("preferences", None);
            let application_handle = application.clone();

            preferences_action.connect_activate(move |_, _| {
                if let Some(active_window) = application_handle.active_window() {
                    show_preferences_dialog(&active_window);
                }
            });


            let shortcuts_action = SimpleAction::new("shortcuts", None);
            let application_handle = application.clone();

            shortcuts_action.connect_activate(move |_, _| {
                if let Some(active_window) = application_handle.active_window() {
                    show_shortcuts_dialog(&active_window);
                }
            });


            let about_action = SimpleAction::new("about", None);
            let application_handle = application.clone();

            about_action.connect_activate(move |_, _| {
                if let Some(active_window) = application_handle.active_window() {
                    show_about_dialog(&active_window);
                }
            });


            let close_action = gio::SimpleAction::new("close", None);
            let application_handle = application.clone();

            close_action.connect_activate(move |_, _| {
                if let Some(window) = application_handle.active_window() {
                    // TODO: Remove database from memory here
                    window.close();
                }
            });


            let quit_action = gio::SimpleAction::new("quit", None);
            let application_handle = application.clone();

            quit_action.connect_activate(move |_, _| {
                application_handle.quit();
            });


            let open_action = gio::SimpleAction::new("open", None);
            let application_handle = application.clone();

            open_action.connect_activate(move |_, _| {
                if let Some(parent) = application_handle.active_window() {
                    show_file_dialog(&parent);
                }
            });


            application.add_action(&preferences_action);
            application.add_action(&shortcuts_action);
            application.add_action(&about_action);
            application.add_action(&close_action);
            application.add_action(&quit_action);
            application.add_action(&open_action);

            application.set_accels_for_action("app.preferences", &["<Primary>comma"]);
            application.set_accels_for_action("app.shortcuts", &["<Primary>question"]);
            application.set_accels_for_action("app.close", &["<Primary>w"]);
            application.set_accels_for_action("app.quit", &["<Primary>q"]);
            application.set_accels_for_action("app.open", &["<Primary>o"]);
        });


        application.connect_activate(|application| {
            if let Some(window) = application.active_window() {
                window.present();
            } else if let Ok(window) = window_empty_new(application) {
                window.present();
            }
        });


        application.connect_open(move |application, files, _| {
            for file in files {
                if let Some(path) = file.path() {
                    if let Some(window) = application
                        .windows()
                        .iter()
                        .find(|w| w.widget_name().to_string() == path.to_string_lossy())
                    {
                        window.present();
                    } else {
                        try_window_new(application, file, true);
                    }
                }
            }
        });

        application.run();

        Ok(())
    }
}
