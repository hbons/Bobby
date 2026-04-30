//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gio::File;

use gtk4::prelude::*;
use gtk4::{
    Orientation,
    ScrolledWindow,
    Widget,
    Window,
};

use libadwaita::prelude::*;
use libadwaita::{
    Application,
    ApplicationWindow,
    HeaderBar,
    StatusPage,
    ToastOverlay,
    ToolbarStyle,
    ToolbarView,
};

use crate::bobby::prelude::*;

use crate::gtk::actions::prelude::*;
use crate::gtk::util::widget_by_name;
use crate::gtk::widgets::button::button_open_new;
use crate::gtk::widgets::content::content_new;
use crate::gtk::widgets::drop_target::drop_target_new;
use crate::gtk::widgets::menu::main_menu_new;
use crate::gtk::widgets::switcher::table_switcher_new;


pub fn window_handle_open(
    application: &Application,
    file: &File,
    table_name: Option<String>,
) -> Result<(), Box<dyn Error>>
{
    let path = file
        .path()
        .ok_or("Selected file has no local path")?
        .to_string_lossy()
        .to_string();

    let window = application
        .windows()
        .iter()
        .find(|w| w.widget_name() == path)
        .cloned();

    if let Some(w) = window {
        w.present();
        return Ok(());
    }


    let settings = gio::Settings::new("studio.planetpeanut.Bobby"); // TODO

    let row_order = match settings.string("row-order").as_str() {
        "newest-first" => Some(RowOrder::Descending),
        "oldest-first" => Some(RowOrder::Ascending),
        _ => None,
    };


    for window in application.windows() {
        if window.widget_name() == IS_EMPTY_WINDOW {
            match Database::from_file(file, row_order) {
                Ok(db) => window_show_content_state(&window, &db, table_name.clone())?,
                Err(e) => window_show_error_state(&window, file, e)?,
            }

            window.present();
            return Ok(());
        }
    }

    let window = window_new(application, Some(file), table_name.clone())?;
    window.present();

    Ok(())
}


pub const IS_EMPTY_WINDOW: &str = "1";

pub fn window_new(
    application: &Application,
    file: Option<&File>,
    table_name: Option<String>,
) -> Result<ApplicationWindow, Box<dyn Error>>
{
    let window = ApplicationWindow::builder()
        .application(application)
        .default_width(960)
        .default_height(640)
        .build();

    let header = HeaderBar::new();
    header.set_widget_name("header_bar");
    header.pack_end(&main_menu_new(application));

    let toolbar_view = ToolbarView::new();
    toolbar_view.set_widget_name("toolbar_view");
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_top_bar_style(ToolbarStyle::Flat);
    window.add_css_class("flat");

    window.set_content(Some(&toolbar_view));
    window.add_controller(drop_target_new(&window));
    window.add_action(&close_action(&window));


    let settings = gio::Settings::new("studio.planetpeanut.Bobby"); // TODO

    let row_order = match settings.string("row-order").as_str() {
        "newest-first" => Some(RowOrder::Descending),
        "oldest-first" => Some(RowOrder::Ascending),
        _ => None,
    };


    let app_window = window.clone();
    let window = window.upcast::<Window>();

    match file {
        Some(f) =>
            match Database::from_file(f, row_order) {
                Ok(db) => window_show_content_state(&window, &db, table_name)?,
                Err(e) => window_show_error_state(&window, f, e)?,
            },
        None => window_show_empty_state(&window)?,
    };

    // window.connect_close_request({
    //     // TODO: Remove views and Database from memory on close
    // });

    Ok(app_window)
}


pub fn window_show_empty_state(
    window: &Window,
) -> Result<(), Box<dyn Error>>
{
    let title = "Bobby".to_string();

    let page = StatusPage::builder()
        .icon_name("studio.planetpeanut.Bobby-symbolic")
        .title("Browse Databases")
        .description("Drag and drop <b>SQLite files</b> here")
        .child(&button_open_new(window))
        .hexpand(true)
        .vexpand(true)
        .build();

    window.set_title(Some(&title));
    window.set_widget_name(IS_EMPTY_WINDOW);
    window_set_child(window, &page)?;

    Ok(())
}


fn window_show_error_state(
    window: &Window,
    file: &File,
    error: Box<dyn Error>,
) -> Result<(), Box<dyn Error>>
{
    let title = file
        .path()
        .ok_or("Missing path")?
        .file_name()
        .ok_or("Missing file name")?
        .to_string_lossy()
        .to_string();

    let page = StatusPage::builder()
        .icon_name("dialog-error-symbolic")
        .title("Unable to Open File")
        .description(error.to_string())
        .child(&button_open_new(window))
        .hexpand(true)
        .vexpand(true)
        .build();

    let path = file
        .path()
        .ok_or("Selected file has no local path")?
        .to_string_lossy()
        .to_string();

    window.set_title(Some(&title));
    window.set_widget_name(&path);
    window_set_child(window, &page)?;

    Ok(())
}


fn window_show_content_state(
    window: &Window,
    db: &Database,
    table_name: Option<String>,
) -> Result<(), Box<dyn Error>>
{
    // SAFETY: Window outlives the database
    unsafe {
        window.set_data("db", db.clone());
    }

    let path = db.file
        .path()
        .ok_or("Selected file has no local path")?
        .to_string_lossy()
        .to_string();

    let title = db.file.path()
        .ok_or("Missing file")?
        .file_name()
        .ok_or("Missing file name")?
        .to_string_lossy()
        .to_string();


    let tables = db.tables()?;

    let table =
        if let Some(name) = table_name {
            tables
                .iter()
                .find(|t| t.name() == name)
                .cloned()
                .ok_or("Table does not exist")?
        } else {
            tables
                .first()
                .cloned()
                .ok_or("Table list empty")?
        };

    let table_index = tables
        .iter()
        .position(|t| t.name() == table.name())
        .ok_or("Table does not exist")?
        .to_string();


    let content = content_new(db, &table)?;

    let banner = libadwaita::Banner::builder()
        .title("File has changed")
        .button_label("Reload")
        .action_name("app.reload")
        .button_style(libadwaita::BannerButtonStyle::Suggested)
        // .revealed(true) // TODO: Reveal on file changes
        .build();

    let layout = gtk4::Box::new(Orientation::Vertical, 0);
    layout.append(&banner);
    layout.append(&content);

    let overlay = ToastOverlay::new();
    overlay.set_child(Some(&layout));


    let switcher = table_switcher_new(&tables);
    switcher.set_label(&table.name());

    let widget = widget_by_name(
        "header_bar",
        window.upcast_ref::<Widget>(),
    ).ok_or("Missing widget named 'header_bar'")?;

    let header = widget.downcast::<HeaderBar>()
        .map_err(|w|
            format!(
                "Expected HeaderBar, but got {}",
                w.type_().name()
            )
        )?;

    header.pack_start(&switcher);


    let widget = widget_by_name(
        "toolbar_view",
        window.upcast_ref::<Widget>(),
    ).ok_or("Missing widget named 'toolbar_view'")?;

    let toolbar_view = widget.downcast::<ToolbarView>()
        .map_err(|w|
            format!(
                "Expected ToolbarView, but got {}",
                w.type_().name()
            )
        )?;

    toolbar_view.set_top_bar_style(ToolbarStyle::RaisedBorder);


    window.set_title(Some(&title));
    window.set_widget_name(&path);
    window_set_child(window, &overlay)?;

    let window = window.downcast_ref::<ApplicationWindow>()
        .ok_or("Could not cast to ApplicationWindow")?;

    window.add_action(&copy_row_action(window, &overlay));
    window.add_action(&copy_val_action(window, &overlay));
    window.add_action(&switch_table_action(window, layout, table_index, tables, switcher)); // TODO: Ugly

    Ok(())
}


fn window_set_child(
    window: &Window,
    child: &impl IsA<Widget>,
) -> Result<(), Box<dyn Error>>
{
    let widget = widget_by_name(
        "toolbar_view",
        window.upcast_ref::<Widget>()
    ).ok_or("Missing widget named 'toolbar_view'")?;

    if let Ok(toolbar_view) = widget.downcast::<ToolbarView>() {
        toolbar_view.set_content(
            Some(child)
        );
    }

    Ok(())
}


pub fn window_change_content(
    window: &ApplicationWindow,
    table: &Table,
) -> Result<ScrolledWindow, Box<dyn Error>>
{
    // SAFETY: Window outlives the database
    let db = unsafe {
        window
            .data::<Database>("db")
            .map(|db| db.as_ref())
    };

    let content = content_new(
        db.ok_or("Database not found on window")?,
        table
    )?;

    // TODO: Swap the content here. Need to get the layout box somehow...

    Ok(content)
}
