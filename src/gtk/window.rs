//   Bobby, browse SQLite files
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it
//   under the terms of the GNU General Public License v3 or any later version.


use std::cell::Ref;
use std::error::Error;
use std::path::Path;

use gio::SimpleAction;

use gio::glib::BoxedAnyObject;
use gtk4::prelude::*;
use gtk4::{
    gdk::DragAction,
    gdk::Display,
    gdk::FileList,
    glib::Variant,
    glib::VariantTy,
    Align,
    Button,
    ColumnView,
    DropTarget,
    Orientation,
    ScrolledWindow,
    SingleSelection,
    Widget,
};

use libadwaita::prelude::*;
use libadwaita::{
    Application,
    ApplicationWindow,
    HeaderBar,
    StatusPage,
    // Toast,
    // ToastOverlay,
    // ToastPriority,
};

use crate::bobby::prelude::*;

use super::content::content_new;
use super::file::open_file_dialog;
use super::menu::main_menu_new;
use super::switcher::table_switcher_new;


const EMPTY_WINDOW: &str = "empty-window";

pub fn window_empty_new(application: &Application) -> Result<ApplicationWindow, Box<dyn Error>> {
    let window = ApplicationWindow::builder()
        .title("Bobby")
        .application(application)
        .default_width(600)
        .default_height(500)
        .build();

    let header = HeaderBar::new();
    header.add_css_class("flat");
    header.pack_end(&main_menu_new(application));

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
    window.set_widget_name(EMPTY_WINDOW);
    window.add_controller(drop_target_new(&window));

    Ok(window)
}


fn drop_target_new(window: &ApplicationWindow) -> DropTarget {
    let drop_target = DropTarget::new(
        FileList::static_type(),
        DragAction::COPY,
    );

    let window_handle = window.clone();

    drop_target.connect_drop(move |_, value, _, _| {
        if window_handle.widget_name() == EMPTY_WINDOW {
            // Keep reference alive
            window_handle.set_visible(false);
        }

        if let Some(application) = window_handle.application() {
            if let Ok(list) = value.get::<FileList>() {
                application.open(&list.files(), "");
            }
        }

        if window_handle.widget_name() == EMPTY_WINDOW {
            window_handle.close();
        }

        true
    });

    drop_target
}


fn button_open_new(window: &ApplicationWindow) -> Button {
    let button = Button::builder()
        .label("Open...")
        .css_classes(["pill", "suggested-action"])
        .halign(Align::Center)
        .build();

    let window_weak = window.downgrade();

    button.connect_clicked(move |_| {
        if let Some(window) = window_weak.upgrade() {
            open_file_dialog(&window);
        }
    });

    button
}


pub fn window_new(application: &Application, path: &Path, table_name: Option<String>) -> Result<ApplicationWindow, Box<dyn Error>> {
    let settings = gio::Settings::new("studio.planetpeanut.Bobby"); // TODO

    let row_order = match settings.string("row-order").as_str() {
        "newest-first" => Some(RowOrder::Descending),
        "oldest-first" => Some(RowOrder::Ascending),
        _ => None,
    };

    let db = Database::from_file(path, row_order)?;
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

    let title = &path.file_name()
        .ok_or("err")?
        .to_string_lossy()
        .to_string();

    let window = ApplicationWindow::builder()
        .application(application)
        .title(title)
        .default_width(960)
        .default_height(640)
        .build();

    let switcher = table_switcher_new(&tables);
    switcher.set_label(&table.name());

    let main_menu = main_menu_new(application);

    let header = HeaderBar::new();
    header.pack_start(&switcher);
    header.pack_end(&main_menu);


    let content = content_new(&db, &table);

    let table_index = tables
        .iter()
        .position(|t| t.name() == table.name())
        .ok_or("Table does not exist")?
        .to_string();

    let table_action = SimpleAction::new_stateful(
        "table",
        Some(&String::static_variant_type()),
        &Variant::from(table_index),
    );

    let layout = gtk4::Box::new(Orientation::Vertical, 0);


    let window_handle = window.clone();
    let switcher_handle = switcher.clone();
    let layout_handle = layout.clone();

    // TODO: Move to actions.rs
    table_action.connect_change_state(move |action, value| {
        if let Some(v) = value {
            action.set_state(v);
        }

        if let Some(table) = value
            .and_then(|v| v.str())
            .and_then(|s| s.parse::<usize>().ok())
            .and_then(|i| tables.get(i))
        {
            switcher_handle.set_label(&table.name());

            match window_change_content(&window_handle, table) {
                Ok(new_content) => {
                    if let Some(old_content) = layout_handle.last_child() {
                        layout_handle.remove(&old_content);
                        layout_handle.append(&new_content);
                    }
                },
                Err(e) => eprintln!("Could not change content: {e}"),
            };
        }
    });

    layout.append(&header);
    layout.append(&content);

    window.set_content(Some(&layout));
    window.add_controller(drop_target_new(&window));


    let window_handle = window.clone();
    let copy_val_action = gio::SimpleAction::new("copy-val", Some(VariantTy::STRING));

    // TODO: Move to actions.rs
    copy_val_action.connect_activate(move |_, row_col_index| {
        if let Some(column_view) = find_column_view(window_handle.upcast_ref()) {
            if let Some((row_index, col_index)) = row_col_index
                .and_then(|v| v.str())
                .and_then(|s| s.split_once(':'))
            {
                let row_index = row_index.parse::<usize>().unwrap_or_default();
                let col_index = col_index.parse::<usize>().unwrap_or_default();

                if let Some(row) = get_row(column_view, row_index) {
                    if let Some(cell) = row.cells.get(col_index) {
                        _ = copy_to_clipboard(&cell.to_string()); // TODO
                    }
                }
            }
        }
    });


    let window_handle = window.clone();
    let copy_row_action = SimpleAction::new("copy-row", Some(VariantTy::STRING));

    // TODO: Move to actions.rs
    copy_row_action.connect_activate(move |_, row_index| {
        if let Some(column_view) = find_column_view(window_handle.upcast_ref()) {
            if let Some(row) = row_index
                .and_then(|v| v.str())
                .and_then(|s| s.parse::<usize>().ok())
                .and_then(|i| get_row(column_view, i))
            {
                let settings = gio::Settings::new("studio.planetpeanut.Bobby"); // TODO

                let separator = settings.string("column-separator");
                let separator = separator.as_str().parse::<ColumnSeparator>();

                _ = copy_to_clipboard(
                    &row.format_with(separator.unwrap_or_default())
                );
            }
        }
    });


    window.add_action(&copy_val_action);
    window.add_action(&copy_row_action);
    window.add_action(&table_action);

    // SAFETY: Window outlives the database
    unsafe {
        window.set_data("db", db);
    }

    Ok(window)
}


fn copy_to_clipboard(s: &str) -> Result<(), Box<dyn Error>> {
    let display = Display::default().ok_or("Missing Display")?;
    let clipboard = display.clipboard();
    clipboard.set_text(s);

    Ok(())
}


fn window_change_content(window: &ApplicationWindow, table: &Table)
    -> Result<ScrolledWindow, Box<dyn Error>> {

    // SAFETY: Window outlives the database
    let db = unsafe {
        window
            .data::<Database>("db")
            .map(|db| db.as_ref())
    };

    let db = db.ok_or("Database not found on window")?;
    let content = content_new(db, table);

    // TODO: Swap the content here. Need to get the layout box somehow...

    Ok(content)
}


fn find_column_view(root: &Widget) -> Option<ColumnView> {
    if let Ok(column_view) = root.clone().downcast::<ColumnView>() {
        return Some(column_view);
    }

    let mut child = root.first_child();

    while let Some(widget) = child {
        if let Some(found) = find_column_view(&widget) {
            return Some(found);
        }

        child = widget.next_sibling();
    }

    None
}


fn get_row(column_view: ColumnView, position: usize) -> Option<Row> {
    let model = column_view.model()?;
    let selection = model.downcast_ref::<SingleSelection>()?;
    let item = selection
        .item(position as u32)
        .and_then(|o| o.downcast::<BoxedAnyObject>().ok())?;

    let row: Ref<Row> = item.borrow();
    let row = row.clone();

    Some(row)
}
