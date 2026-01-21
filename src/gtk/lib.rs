//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gio::{
    Menu,
    SimpleAction,
};

use gtk4::prelude::*;
use gtk4::gio::ApplicationFlags;

use libadwaita::Application;

use crate::app::App;
use crate::gui::Gui;

use super::window::{
    window_empty_new,
    window_new,
};

use super::preferences::show_preferences_dialog;
use super::about::show_about_dialog;


impl Gui for App {
    // Docs: https://docs.gtk.org/gtk4
    //       https://gnome.pages.gitlab.gnome.org/libadwaita/doc

    fn gui_run(&self) -> Result<(), Box<dyn Error>> {
        let application = Application::builder()
            .application_id(&self.id)
            .flags(ApplicationFlags::HANDLES_OPEN)
            .build();

        application.set_accels_for_action("app.preferences", &["<Control>comma"]);
        // application.set_accels_for_action("app.close", &["<Ctrl>w"]); // TODO
        // application.set_accels_for_action("app.quit", &["<Ctrl>q"]); // TODO

        application.connect_startup(|application| {
            let preferences_action = SimpleAction::new("preferences", None);
            let application_handle = application.clone();

            preferences_action.connect_activate(move |_, _| {
                if let Some(active_window) = application_handle.active_window() {
                    show_preferences_dialog(&active_window);
                }
            });


            let about_action = SimpleAction::new("about", None);
            let application_handle = application.clone();

            about_action.connect_activate(move |_, _| {
                if let Some(active_window) = application_handle.active_window() {
                    show_about_dialog(&active_window);
                }
            });


            application.add_action(&preferences_action);
            application.add_action(&about_action);

            let menu = Menu::new();
            menu.append(Some("Preferences"), Some("app.preferences"));
            menu.append(Some("About Bobby"), Some("app.about"));

            unsafe { application.set_data("menu", menu); }
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
                        if let Ok(window) = window_new(&application, path.as_path(), None, false) {
                            window.present();
                        }
                    }
                }
            }
        });


        application.run();
        Ok(())
    }
}
