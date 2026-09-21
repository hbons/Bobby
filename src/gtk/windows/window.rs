//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::error::Error;

use gio::File;
use gtk4::{
    glib::Propagation,
    glib::Variant,
    gdk::Key,
    ColumnView,
    ColumnViewColumn,
    EventControllerKey,
    MenuButton,
    Orientation,
    ScrolledWindow,
    SearchEntry,
    ToggleButton,
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
use crate::gtk::widgets::button::{ button_open_new, button_search_new };
use crate::gtk::widgets::content::{ content_new, content_force_redraw };
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

    for window in application.windows() {
        if window.widget_name() == IS_EMPTY_WINDOW {
            match Database::from_file(file, row_order_from_settings()) {
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


pub fn window_reload(
    application: &Application,
    file: &File,
) -> Result<(), Box<dyn Error>>
{
    let path = file
        .path()
        .ok_or("Selected file has no local path")?
        .to_string_lossy()
        .to_string();

    let window = &application
        .active_window()
        .ok_or("Missing active window")?;

    let switcher =
        widget_by_name::<MenuButton>("switcher", window)
            .ok_or("Missing MenuButton named 'switcher'")?;

    let table_name: Option<String> = switcher.label()
        .map(|g| g.into());


    // Remember the scroll position
    let scrolled_window = widget_by_name::<ScrolledWindow>(
        "content",
        window,
    ).ok_or("Missing widget named 'content'")?;

    let h_value = scrolled_window.hadjustment().value();
    let v_value = scrolled_window.vadjustment().value();

    for window in application.windows() {
        if window.widget_name() == path {
            match Database::from_file(file, row_order_from_settings()) {
                Ok(db) => window_show_content_state(&window, &db, table_name)?,
                Err(e) => window_show_error_state(&window, file, e)?,
            }

            // Reapply scroll position
            let scrolled_window =
                widget_by_name::<ScrolledWindow>(
                    "content",
                    &window,
                ).ok_or("Missing widget named 'content'")?;

            gtk4::glib::idle_add_local_once(move || {
                scrolled_window.hadjustment().set_value(h_value);
                scrolled_window.vadjustment().set_value(v_value);
            });

            return Ok(());
        }
    }

    Ok(())
}


fn row_order_from_settings() -> Option<RowOrder> {
    let settings = gio::Settings::new("studio.planetpeanut.Bobby"); // TODO

    match settings.string("row-order").as_str() {
        "newest-first" => Some(RowOrder::Descending),
        "oldest-first" => Some(RowOrder::Ascending),
        _ => None,
    }
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
    // toolbar_view.add_top_bar(&bar);
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


    let header =
        widget_by_name::<HeaderBar>(
            "header_bar",
            window,
        ).ok_or("Missing HeaderBar named 'header_bar'")?;

    let switcher =
        widget_by_name::<MenuButton>(
            "switcher",
            window,
        ).unwrap_or_else(|| {
                let switcher = table_switcher_new(&tables);
                header.pack_start(&switcher);
                switcher
            }
        );

    switcher.set_label(&table.name());


    let toolbar_view =
        widget_by_name::<ToolbarView>(
            "toolbar_view",
            window,
        ).ok_or("Missing ToolbarView named 'toolbar_view'")?;

    toolbar_view.set_top_bar_style(ToolbarStyle::RaisedBorder);


    let search_button =
        widget_by_name::<ToggleButton>(
            "search_button",
            window,
        ).unwrap_or(
            button_search_new(window)
        );

    if search_button.parent().is_none() {
        header.pack_end(&search_button);
    }

    window.set_title(Some(&title));
    window.set_widget_name(&path);
    window_set_child(window, &overlay)?;

    let window = window.downcast_ref::<ApplicationWindow>()
        .ok_or("Could not cast to ApplicationWindow")?;

    window.add_action(&copy_row_action(window, &overlay));
    window.add_action(&copy_val_action(window, &overlay));
    window.add_action(&reload_action(window));
    window.add_action(&switch_table_action(window, &layout, &table_index, &tables, &switcher)); // TODO: Ugly
    window.add_action(&search_table_action(window, &layout, &table_index, &tables)); // TODO: Ugly
    window.add_action(&search_toggle_action(window));

    Ok(())
}


fn window_set_child(
    window: &Window,
    child: &impl IsA<Widget>,
) -> Result<(), Box<dyn Error>>
{
    let toolbar_view =
        widget_by_name::<ToolbarView>(
            "toolbar_view",
            window,
        ).ok_or("Missing ToolbarView named 'toolbar_view'")?;

    toolbar_view.set_content(Some(child));

    Ok(())
}


pub fn window_toggle_row_numbers(window: &Window) -> Result<(), Box<dyn Error>> {
    let scrolled_window =
        widget_by_name::<ScrolledWindow>(
            "content",
            window,
        ).ok_or("Missing ScrolledWindow named 'content'")?;

    let widget = scrolled_window
        .child()
        .ok_or("Missing ScrolledWindow child")?;

    let column_view = widget.downcast::<ColumnView>()
        .map_err(|w|
            format!(
                "Expected ColumnView, but got {}",
                w.type_().name()
            )
        )?;

    content_force_redraw(&column_view);

    let settings = gio::Settings::new("studio.planetpeanut.Bobby"); // TODO

    if let Some(obj) = column_view.columns().item(0) {
        if let Some(first_col) = obj.downcast_ref::<ColumnViewColumn>() {
            first_col.set_visible(
                settings.boolean("row-numbers")
            );
        }
    }

    Ok(())
}


pub fn window_toggle_search(window: &Window) -> Result<(), Box<dyn Error>> {
    let header = widget_by_name::<HeaderBar>("header_bar", window)
        .ok_or("Missing HeaderBar named 'header_bar'")?;

    let button = widget_by_name::<ToggleButton>("search_button", window)
        .ok_or("Missing ToggleButton named 'search_button'")?;

    let entry_option = widget_by_name::<SearchEntry>("search_entry", window);


    if header.title_widget().is_none() {
        button.set_active(true);
    } else {
        button.set_active(false);
        header.set_title_widget(None::<&Widget>);

        if let Some(entry) = &entry_option {
            entry.set_text("");
            let table_index = window_selected_table_index(&window);

            _ = window.activate_action(
                "win.search",
                Some(&Variant::from(table_index.to_string())),
            );
        }
    }

    if let Some(entry) = entry_option {
        entry.grab_focus();
    } else {
        let entry = SearchEntry::new();
        entry.set_widget_name("search_entry");

        let controller = EventControllerKey::new();

        let header_clone = header.clone();
        let button_clone = button.clone();
        let entry_clone = entry.clone();

        controller.connect_key_pressed(
            move |_, key, _, _| {
                if key == Key::Escape {
                    // First ESC clears the entry
                    if entry_clone.text() != "" {
                        entry_clone.set_text("");
                        return Propagation::Stop;
                    }

                    header_clone.set_title_widget(None::<&Widget>);
                    button_clone.set_active(false);
                    entry_clone.set_text("");

                    Propagation::Stop
                } else {
                    Propagation::Proceed
                }
            }
        );

        let window_clone = window.clone();

        entry.add_controller(controller);
        entry.connect_search_changed(move |_entry| {
            let table_index = window_selected_table_index(&window_clone);

            _ = window_clone.activate_action(
                "win.search",
                Some(&Variant::from(table_index.to_string())),
            ); // TODO: async search to fix UI blocking
        });

        header.set_title_widget(Some(&entry));
        entry.grab_focus();
    }

    Ok(())
}


pub fn window_search_text(window: &ApplicationWindow) -> Option<String> {
    let entry = widget_by_name::<SearchEntry>("search_entry", window);
    entry.map(|e| e.text().to_string())
}


pub fn window_selected_table_index(window: &Window) -> usize {
    let switcher = widget_by_name::<MenuButton>("switcher", window).unwrap();
    let table_name = switcher.label();

    let db = unsafe {
        window
            .data::<Database>("db")
            .map(|db| db.as_ref())
    };

    let tables = db.unwrap().tables().unwrap();

    let table =
        if let Some(name) = table_name {
            tables
                .iter()
                .find(|t| t.name() == name)
                .cloned()
                .unwrap_or_default()
        } else {
            tables
                .first()
                .cloned()
                .ok_or("Table list empty")
                .unwrap()
        };

    tables
        .iter()
        .position(|t| t.name() == table.name())
        .unwrap_or(0)
}


pub fn window_toggle_row_order(window: &Window) -> Result<(), Box<dyn Error>> {
    _ = window.activate_action("win.reload", None);
    Ok(())
}

pub fn window_toggle_monospace_font(window: &Window) -> Result<(), Box<dyn Error>> {
    _ = window.activate_action("win.reload", None);
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

    // TODO: Search should clear before switching tables
    // if window_search_text(window).is_some() {
    //     let window = window.clone().upcast::<Window>();
    //     _ = window.activate_action("win.search_toggle", None);
    // }

    let content = content_new(
        db.ok_or("Database not found on window")?,
        &table
    )?;

    // TODO: Swap the content here. Need to get the layout box somehow...

    Ok(content)
}
