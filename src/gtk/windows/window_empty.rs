//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use libadwaita::prelude::*;
use libadwaita::{
    Application,
    ApplicationWindow,
    HeaderBar,
    StatusPage,
    ToolbarView,
};

use crate::gtk::widgets::button::button_open_new;
use crate::gtk::widgets::drop_target::drop_target_new;
use crate::gtk::widgets::menu::main_menu_new;


pub const IS_EMPTY_WINDOW: &str = "1";

pub fn window_empty_new(application: &Application) -> Result<ApplicationWindow, Box<dyn Error>> {
    let window = ApplicationWindow::builder()
        .title("Bobby")
        .application(application)
        .default_width(600)
        .default_height(500)
        .build();

    // window.add_css_class("devel"); // TODO

    let menu = &main_menu_new(application);

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

    let toolbar_view = ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&page));

    window.set_content(Some(&toolbar_view));
    window.set_widget_name(IS_EMPTY_WINDOW);
    window.add_controller(drop_target_new(&window));

    Ok(window)
}
