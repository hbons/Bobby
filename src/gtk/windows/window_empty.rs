//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gtk4::prelude::*;
use gtk4::{
    Align,
    Button,
    Orientation,
};

use libadwaita::prelude::*;
use libadwaita::{
    Application,
    ApplicationWindow,
    HeaderBar,
    StatusPage,
};

use super::super::drop_target::drop_target_new;
use super::super::menu::main_menu_new;


pub const IS_EMPTY_WINDOW: &str = "1";

pub fn window_empty_new(application: &Application) -> Result<ApplicationWindow, Box<dyn Error>> {
    let window = ApplicationWindow::builder()
        .title("Bobby")
        .application(application)
        .default_width(600)
        .default_height(500)
        .build();

    // window.add_css_class("devel"); // TODO

    let menu = &main_menu_new();

    let header = HeaderBar::new();
    header.add_css_class("flat");
    header.pack_end(menu);

    let page = StatusPage::builder()
        .icon_name("studio.planetpeanut.Bobby-symbolic")
        .title("Browse Databases")
        .description("Drag and drop <b>SQLite files</b> here")
        .child(&button_open_new(&window))
        .hexpand(true)
        .vexpand(true)
        .build();

    let layout = gtk4::Box::new(Orientation::Vertical, 0);
    layout.append(&header);
    layout.append(&page);

    window.set_content(Some(&layout));
    window.set_widget_name(IS_EMPTY_WINDOW);
    window.add_controller(drop_target_new(&window));

    Ok(window)
}


pub fn button_open_new(window: &ApplicationWindow) -> Button {
    let button = Button::builder()
        .label("Open...")
        .css_classes(["pill", "suggested-action"])
        .halign(Align::Center)
        .build();

    let window_weak = window.downgrade();

    button.connect_clicked(move |_| {
        if let Some(window) = window_weak.upgrade() {
            if let Some(application) = window.application() {
                application.activate_action("open", None);
            }
        }
    });

    button
}
